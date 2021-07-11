use std::cell::RefCell;
use std::convert::TryInto;
use std::io::{self, Read, Write};
use std::iter::Iterator;
use std::rc::Rc;
use std::time::Instant;
use std::{mem, str};

use cpp_core::CppBox;
use qt_core::{AlignmentFlag, QBox, QPtr, QString};
use qt_network::QTcpSocket;
use qt_widgets::QTextBrowser;

use crate::api::Api;
use crate::binding::text::{CharFormat, Cursor};
use crate::binding::{Printable, RColor, RIODevice};
use crate::client::color::Colors;
use crate::client::state::Latest;
use crate::constants::{branding, config};
use crate::escape::{ansi, telnet};
use crate::mxp;
use crate::script::{Callback, PluginHandler, PluginId};
use crate::tr::TrContext;
use crate::ui::{Notepad, Pad};
use crate::world::{AutoConnect, UseMxp, World};

pub mod color;
pub mod state;
mod stream;
use stream::MudStream;
pub mod style;

use color::WorldColor;
use state::{ClientState, Mccp, Phase};
use style::{Style, TextStyle};

#[inline]
fn left<T>(xs: &[T], amt: usize) -> &[T] {
    if xs.len() > amt {
        &xs[..amt]
    } else {
        xs
    }
}

#[derive(Debug)]
pub enum Fragment {
    Html(CppBox<QString>),
    Text(CppBox<QString>),
    Break,
}

impl Fragment {
    pub fn html<S: Printable>(text: S) -> Self {
        Self::Html(text.to_print())
    }

    pub fn text<S: Printable>(text: S) -> Self {
        Self::Text(text.to_print())
    }
}

#[derive(TrContext)]
pub struct Client {
    widget: QPtr<QTextBrowser>,
    cursor: Cursor,
    socket: RIODevice<QTcpSocket>,
    stream: MudStream,
    bufinput: [u8; config::SOCKET_BUFFER],
    bufoutput: Vec<u8>,
    plugins: PluginHandler<Api>,
    world: Rc<World>,
    notepad: Rc<RefCell<Notepad>>,
    phase: Phase,
    style: Style,
    state: ClientState,
    latest: Latest,
}

impl Client {
    /// # Safety
    ///
    /// `widget` and `socket` must be valid.
    pub unsafe fn new(
        widget: QPtr<QTextBrowser>,
        socket: QBox<QTcpSocket>,
        world: Rc<World>,
    ) -> Self {
        let notepad = Rc::new(RefCell::new(Notepad::new(&world.name)));
        let socket = RIODevice::new(socket);
        let api = unsafe {
            Api::new(
                widget.clone(),
                socket.clone(),
                world.clone(),
                notepad.clone(),
            )
        };

        let mut this = Self {
            notepad,
            cursor: unsafe { Cursor::get(&widget) },
            stream: MudStream::new(socket.clone()),
            socket,
            bufinput: [0; config::SOCKET_BUFFER],
            bufoutput: Vec::new(),
            style: Style::new(
                CharFormat::from(unsafe { widget.current_char_format() }),
                world.clone(),
            ),
            widget,
            world,
            phase: Phase::Normal,
            state: ClientState::new(),
            plugins: PluginHandler::new(api),
            latest: Latest::new(),
        };
        this.load_worldscript();
        this
    }

    fn load_worldscript(&mut self) {
        // todo display errors
        match self.world.make_plugin() {
            Ok(Some(plugin)) => mem::drop(self.plugins.load_plugin(plugin)),
            Ok(None) => (),
            Err(_) => (),
        };
    }

    pub fn on_save(&mut self) {
        self.plugins.alter_userdata(|api| api.save_variables());
    }

    pub fn set_world(&mut self, world: Rc<World>) {
        self.style.set_world(world.clone());
        self.plugins.remove(&PluginId::nil()); // remove old worldscript
        self.plugins
            .alter_userdata(|api| api.set_world(world.clone()));
        self.world = world;
        self.load_worldscript();
    }

    pub fn retitle(&mut self, name: &str) {
        self.notepad.borrow_mut().retitle(name);
    }

    pub fn connect(&mut self) {
        let world = &*self.world;
        self.socket.connect(&world.site, world.port);
        self.latest.connected = Instant::now();
    }

    pub fn disconnect(&mut self) {
        // don't want reconnect on manual disconnect
        self.state.disconnect_ok = true;
        // work out how long they were connected
        self.state.total_connect_duration += self.latest.connected.elapsed();
        self.mxp_off(true);
        self.socket.close();
    }

    fn send<S: AsRef<[u8]>>(&self, buf: S) -> io::Result<()> {
        let buf = buf.as_ref();
        self.socket.io().write_all(buf)?;
        Ok(())
    }

    pub fn send_command(&mut self, mut command: String) -> io::Result<()> {
        self.latest.input = Instant::now();
        let world = &*self.world;
        if world.display_my_input {
            if !world.keep_commands_on_same_line && !self.cursor.at_block_start() {
                self.cursor.insert_block();
            }
            let echo_colors = &world.echo_colors;
            self.cursor.insert_text_colored(
                &command,
                Some(&echo_colors.foreground),
                Some(&echo_colors.background),
            );
            self.cursor.insert_block();
            self.scroll_to_bottom();
        }
        command.push('\n');
        self.send(&command)
    }

    pub fn read(&mut self) {
        match self.stream.read(&mut self.bufinput) {
            Ok(0) => (),
            Ok(res) => self.display_msg(self.bufinput[0..res].to_vec()),
            Err(e) => eprintln!("Stream error: {}", e),
        }
    }

    fn scroll_to_bottom(&self) {
        unsafe {
            let scrollbar = self.widget.vertical_scroll_bar();
            scrollbar.set_value(scrollbar.maximum());
        }
    }

    fn insert_html<S: Printable>(&self, text: S) {
        self.cursor.insert_html(text);
    }

    fn insert_text<S: Printable>(&self, text: S) {
        self.cursor.insert_text_formatted(text, self.style.format());
    }

    pub fn print<S: Printable>(&self, text: S) {
        self.insert_text(text);
        self.scroll_to_bottom();
    }

    pub fn println<S: Printable>(&self, text: S) {
        self.cursor.insert_block();
        self.insert_text(text);
        self.scroll_to_bottom();
    }

    #[cfg(feature = "show-special")]
    pub fn append_to_notepad<S: Printable>(&self, kind: Pad, align: AlignmentFlag, text: S) {
        self.notepad.borrow_mut().append(kind, align, text);
    }

    fn flush(&mut self) {
        if !self.bufoutput.is_empty() {
            self.print(&self.bufoutput);
            self.bufoutput.clear();
        }
    }

    fn output_bad_utf8(&mut self) {
        self.bufoutput.append(&mut self.state.utf8_sequence);
    }

    fn init_zlib(&mut self, prepend: Vec<u8>) {
        self.stream.start_decompressing(prepend);
    }

    fn interpret_ansi(&mut self, code: u8) {
        match code {
            ansi::RESET => self.style.reset(),

            ansi::BOLD => self.style.add_flag(TextStyle::Bold),
            ansi::BLINK => self.style.add_flag(TextStyle::Italic),
            ansi::UNDERLINE => self.style.add_flag(TextStyle::Underline),
            ansi::SLOW_BLINK => self.style.add_flag(TextStyle::Italic),
            ansi::FAST_BLINK => self.style.add_flag(TextStyle::Italic),
            ansi::INVERSE => self.style.add_flag(TextStyle::Inverse),
            ansi::STRIKEOUT => self.style.add_flag(TextStyle::Strikeout),

            ansi::CANCEL_BOLD => self.style.remove_flag(TextStyle::Bold),
            ansi::CANCEL_BLINK => self.style.remove_flag(TextStyle::Italic),
            ansi::CANCEL_UNDERLINE => self.style.remove_flag(TextStyle::Underline),
            ansi::CANCEL_SLOW_BLINK => self.style.remove_flag(TextStyle::Italic),
            ansi::CANCEL_FAST_BLINK => self.style.remove_flag(TextStyle::Italic),
            ansi::CANCEL_INVERSE => self.style.remove_flag(TextStyle::Inverse),
            ansi::CANCEL_STRIKEOUT => self.style.remove_flag(TextStyle::Strikeout),

            ansi::FG_256_COLOR => self.phase = Phase::Foreground256Start,
            ansi::BG_256_COLOR => self.phase = Phase::Background256Start,
            _ => {
                if let Some(fg) = WorldColor::fg_from_ansi(code) {
                    self.style.set_foreground(fg);
                } else if let Some(bg) = WorldColor::bg_from_ansi(code) {
                    self.style.set_background(bg);
                }
            }
        }
    }

    // See: https://en.wikipedia.org/wiki/ANSI_escape_code#8-bit
    /// ESC[ 38:5:<n> m Select foreground color
    /// ESC[ 48:5:<n> m Select background color
    /// ESC[ 38;2;<r>;<g>;<b> m Select RGB foreground color
    /// ESC[ 48;2;<r>;<g>;<b> m Select RGB background color
    fn interpret_256_ansi(&mut self, code: u8) {
        fn build_ansi_color(state: &ClientState) -> WorldColor {
            WorldColor::Plain(RColor::rgb(
                state.ansi_red,
                state.ansi_green,
                state.ansi_blue,
            ))
        }
        match self.phase {
            // ESC[ 38: (foreground)
            Phase::Foreground256Start => match code {
                5 => {
                    // 8-bit color
                    self.state.ansi_code = 0;
                    self.phase = Phase::Foreground256Finish;
                }
                2 => {
                    // 24-bit RGB
                    self.state.ansi_code = 0;
                    self.phase = Phase::Foreground24bFinish;
                }
                _ => self.phase = Phase::Normal,
            },
            // ESC[ 48: (background)
            Phase::Background256Start => {
                match code {
                    5 => {
                        // 8-bit color
                        self.state.ansi_code = 0;
                        self.phase = Phase::Background256Finish;
                    }
                    2 => {
                        // 24-bit RGB
                        self.state.ansi_code = 0;
                        self.phase = Phase::Background24bFinish;
                    }
                    _ => self.phase = Phase::Normal,
                }
            }
            Phase::Foreground256Finish => {
                self.style
                    .set_foreground(WorldColor::Xterm(self.state.ansi_code));
                self.phase = Phase::Normal;
            }
            Phase::Background256Finish => {
                self.style
                    .set_background(WorldColor::Xterm(self.state.ansi_code));
                self.phase = Phase::Normal;
            }
            Phase::Foreground24bFinish => {
                self.state.ansi_red = code;
                self.phase = Phase::Foreground24brFinish;
            }
            Phase::Background24bFinish => {
                self.state.ansi_red = code;
                self.phase = Phase::Background24brFinish;
            }
            Phase::Foreground24brFinish => {
                self.state.ansi_green = code;
                self.phase = Phase::Foreground24bgFinish;
            }
            Phase::Background24brFinish => {
                self.state.ansi_green = code;
                self.phase = Phase::Background24bgFinish;
            }
            Phase::Foreground24bgFinish => {
                self.state.ansi_blue = code;
                self.phase = Phase::Foreground24bbFinish;
            }
            Phase::Background24bgFinish => {
                self.state.ansi_blue = code;
                self.phase = Phase::Background24bbFinish;
            }
            Phase::Foreground24bbFinish => {
                self.style.set_foreground(build_ansi_color(&self.state));
                self.phase = Phase::Normal;
            }
            Phase::Background24bbFinish => {
                self.style.set_background(build_ansi_color(&self.state));
                self.phase = Phase::Normal;
            }
            _ => (),
        }
    }

    fn interpret_code(&mut self) {
        if self.phase == Phase::DoingCode {
            self.interpret_ansi(self.state.ansi_code);
        } else {
            self.interpret_256_ansi(self.state.ansi_code);
        }
    }

    fn handle_mxp_error(&self, err: mxp::ParseError) {
        eprintln!("MXP Error: {}", err);
    }

    fn mxp_restore_mode(&mut self) {
        if self.state.mxp_mode == mxp::Mode::SECURE_ONCE {
            self.state.mxp_mode = self.state.mxp_mode_previous;
        }
    }

    fn mxp_off(&mut self, completely: bool) {
        self.style.reset();

        // do nothing else if already off
        if !self.state.mxp_active {
            return;
        }
        // don't close protected tags here
        let closed = match self.state.mxp_active_tags.iter().rposition(|x| x.no_reset) {
            None => 0,
            Some(i) => i + 1,
        };
        self.mxp_close_tags_from(closed);
        self.state.in_paragraph = false;
        self.state.mxp_script = false; // cancel scripts
        self.state.pre_mode = false; // no more preformatted text
        self.state.list_mode = None; // no more ordered/unordered lists
        self.state.list_index = 0;

        if !completely {
            return;
        }
        self.mxp_mode_change(Some(mxp::Mode::OPEN)); // back to open mode
        if self.phase.is_mxp() {
            self.phase = Phase::Normal;
        }
        self.state.pueblo_active = false;
        self.state.mxp_active = false;

        self.plugins.send_to_all(Callback::MxpStop, ());
    }

    fn mxp_on(&mut self, pueblo: bool, manual: bool) {
        // do nothing if already on
        if self.state.mxp_active {
            return;
        }

        self.plugins.send_to_all(Callback::MxpStart, ());

        self.state.mxp_active = true;
        self.state.pueblo_active = pueblo;
        self.state.mxp_script = false;
        self.state.pre_mode = false;
        self.state.last_outstanding_tag_count = 0;
        self.state.list_mode = None;

        // if they turn it on manually we want to leave everything set up
        // (e.g. they turn it off, they turn it on again)
        if manual {
            return;
        }
        // make sure we are back to open as default
        self.state.mxp_mode_default = mxp::Mode::OPEN;
        self.state.mxp_mode = mxp::Mode::OPEN;
        self.state.mxp_active_tags.clear();
        self.state.mxp_elements.clear();
    }

    fn mxp_findtag(&self, was_secure: bool, name: &str) -> Result<usize, mxp::ParseError> {
        for (i, tag) in self.state.mxp_active_tags.iter().enumerate().rev() {
            if tag.name.eq_ignore_ascii_case(name) {
                if !was_secure && tag.secure {
                    return Err(mxp::ParseError::new(
                        name,
                        mxp::Error::TagOpenedInSecureMode,
                    ));
                } else {
                    return Ok(i);
                }
            }
            if !was_secure && tag.secure {
                return Err(mxp::ParseError::new(
                    &tag.name,
                    mxp::Error::OpenTagBlockedBySecureTag,
                ));
            }
        }
        Err(mxp::ParseError::new(name, mxp::Error::OpenTagNotThere))
    }

    fn mxp_endtag(&mut self, tag_body: &str) -> Result<(), mxp::ParseError> {
        let was_secure = self.state.mxp_mode.is_secure();
        self.mxp_restore_mode();
        let mut words = mxp::Words::new(tag_body);
        let name = words.validate_next_or(mxp::Error::InvalidElementName)?;
        // should just have tag name, not </tag blah blah>
        if words.next().is_some() {
            return Err(mxp::ParseError::new(
                tag_body,
                mxp::Error::ArgumentsToClosingTag,
            ));
        }

        let closed = self.mxp_findtag(was_secure, name)?;
        self.mxp_close_tags_from(closed);
        Ok(())
    }

    fn mxp_definition(&mut self, tag: &str) -> Result<(), mxp::ParseError> {
        let was_secure = self.state.mxp_mode.is_secure();
        self.mxp_restore_mode();
        if !was_secure {
            return Err(mxp::ParseError::new(
                &tag,
                mxp::Error::DefinitionWhenNotSecure,
            ));
        }
        let mut words = mxp::Words::new(tag);
        // first word (e.g. ELEMENT or ENTITY)
        let definition = words.validate_next_or(mxp::Error::InvalidDefinition)?;
        let name = words.validate_next_or(mxp::Error::InvalidElementName)?;
        match definition.to_lowercase().as_str() {
            "element" | "el" => self.mxp_make_element(name, words),
            "entity" | "en" => self.mxp_make_entity(name, words),
            "attlist" | "at" => self.mxp_make_attributes(name, words),
            _ => Err(mxp::ParseError::new(
                definition,
                mxp::Error::InvalidDefinition,
            )),
        }
    }

    fn mxp_make_element(&mut self, name: &str, words: mxp::Words) -> Result<(), mxp::ParseError> {
        let args = mxp::Arguments::parse_words(words)?;
        if args.has_keyword(mxp::Keyword::Delete) {
            self.state.mxp_elements.remove(&name);
            return Ok(());
        }
        let el = mxp::Element::parse(name.to_owned(), args)?;
        self.state.mxp_elements.insert(name.to_owned(), el);
        Ok(())
    }

    fn mxp_make_entity(&mut self, key: &str, mut words: mxp::Words) -> Result<(), mxp::ParseError> {
        if mxp::EntityMap::global(key).is_some() {
            return Err(mxp::ParseError::new(key, mxp::Error::CannotRedefineEntity));
        }
        match words.next() {
            Some(body) // once told me
                if !words.any(|word| {
                    word.eq_ignore_ascii_case("delete") || word.eq_ignore_ascii_case("remove")
                }) =>
            {
                let value = self.state.mxp_entities.decode(body)?;
                self.plugins.send_to_all(Callback::MxpSetEntity, format!("{}={}", key, value));
                self.state.mxp_entities.insert(key.to_owned(), value)
            }
            _ => self.state.mxp_entities.remove(key),
        };
        Ok(())
    }

    fn mxp_make_attributes(&mut self, key: &str, words: mxp::Words) -> Result<(), mxp::ParseError> {
        self.state
            .mxp_elements
            .get_mut(key)
            .ok_or_else(|| mxp::ParseError::new(key, mxp::Error::UnknownElementInAttlist))?
            .attributes
            .append(words)
    }

    fn mxp_collected_element(&mut self) -> Result<(), mxp::ParseError> {
        let tag =
            *self.state.mxp_string.get(0).ok_or_else(|| {
                mxp::ParseError::new("collected element", mxp::Error::EmptyElement)
            })?;
        let bytestring = mem::take(&mut self.state.mxp_string);
        let text = str::from_utf8(&bytestring).map_err(|_| {
            mxp::ParseError::new(&format!("{:?}", bytestring), mxp::Error::MalformedBytes)
        })?;

        match tag {
            b'!' => self.mxp_definition(&text[1..]),
            b'/' => self.mxp_endtag(&text[1..]),
            _ => self.mxp_start_tag(&text),
        }
    }

    fn mxp_start_tag(&mut self, tag: &str) -> Result<(), mxp::ParseError> {
        self.flush(); // probably going to change style or insert HTML
        let secure = self.state.mxp_mode.is_secure();
        self.mxp_restore_mode();
        let mut words = mxp::Words::new(tag);
        let name = words.validate_next_or(mxp::Error::InvalidElementName)?;
        let component = self.state.mxp_elements.get_component(&name)?;
        let flags = component.flags();
        self.state.mxp_active_tags.push(mxp::Tag {
            name: name.to_owned(),
            secure,
            no_reset: flags.contains(mxp::TagFlag::NoReset),
            span_index: self.style.len(),
        });
        if !flags.contains(mxp::TagFlag::Open) && !secure {
            return Err(mxp::ParseError::new(
                &name,
                mxp::Error::ElementWhenNotSecure,
            ));
        }
        let argstring = words.as_str();
        let mut args = mxp::Arguments::parse_words(words)?;

        // call script if required for user-defined elements
        if name != "afk"
            && self.plugins.send_to_all_until(
                Callback::MxpOpenTag,
                (format!("{},{}", name, argstring), &args),
                enums![true],
            )
        {
            return Ok(());
        }

        if !flags.contains(mxp::TagFlag::Command) {
            // TODO do nothing?
        }

        let mut span = self.style.span().child();
        span.variable = component.variable();
        let mut fragments = Vec::new();
        let unchanged = span.clone();

        match component {
            mxp::ElementComponent::Atom(atom) => {
                for value in args.values_mut() {
                    *value = self.state.mxp_entities.decode(value)?;
                }
                self.mxp_open_atom(&mut span, &mut fragments, atom.action, args);
            }
            mxp::ElementComponent::Custom(el) => {
                // create a temporary vector to avoid borrow conflict
                // could clone the element instead, but that seems like a waste
                let actions: Result<Vec<_>, _> = el
                    .items
                    .iter()
                    .map(|item| {
                        let mut newargs = mxp::Arguments::new();
                        for (i, arg) in &item.arguments {
                            let val = self.state.mxp_entities.decode_el(el, arg, &args)?;
                            match i {
                                mxp::ArgumentIndex::Positional(..) => newargs.push(val),
                                mxp::ArgumentIndex::Named(key) => newargs.set(key, val),
                            }
                        }
                        Ok((item.atom.action, newargs))
                    })
                    .collect();
                for (action, newargs) in actions? {
                    self.mxp_open_atom(&mut span, &mut fragments, action, newargs);
                }
            }
        }

        if span != unchanged {
            self.style.push(span);
        }

        for fragment in fragments {
            match fragment {
                Fragment::Html(text) => self.insert_html(text),
                Fragment::Text(text) => self.insert_text(text),
                Fragment::Break => self.cursor.insert_block(),
            }
        }
        self.scroll_to_bottom();

        Ok(())
    }

    pub fn mxp_open_atom(
        &mut self,
        span: &mut mxp::Span,
        fragments: &mut Vec<Fragment>,
        mut action: mxp::Action,
        args: mxp::Arguments,
    ) {
        use mxp::{Action, Atom, InList, Keyword, Link, SendTo};
        let world = &*self.world;
        let get_color = |name: &str| {
            if world.ignore_mxp_color_changes {
                None
            } else {
                Colors::named(name)
            }
        };
        // special processing for Pueblo
        // a tag like this: <A XCH_CMD="examine #1">
        // will convert to a SEND tag
        if action == Action::Hyperlink {
            if args.get("xch_cmd").is_some() {
                self.state.pueblo_active = true; // for correct newline processing
                action = Action::Send;
            }
        }
        match action {
            // temporarily make headlines the same as bold
            Action::H1 | Action::H2 | Action::H3 | Action::H4 | Action::H5 | Action::H6 => {
                span.flags.insert(TextStyle::Bold);
            }
            Action::Bold => {
                span.flags.insert(TextStyle::Bold);
            }
            Action::Underline => {
                span.flags.insert(TextStyle::Underline);
            }
            Action::Italic => {
                span.flags.insert(TextStyle::Italic);
            }
            Action::Color => {
                let mut scanner = args.scan();
                if let Some(fg) = scanner.next_or(&["fore"]) {
                    if let Some(fg) = get_color(fg) {
                        span.foreground = Some(fg.into_owned());
                    }
                }
                if let Some(bg) = scanner.next_or(&["back"]) {
                    if let Some(bg) = get_color(bg) {
                        span.background = Some(bg.into_owned());
                    }
                }
            }
            Action::High => {
                span.foreground = Some(world.color(self.style.foreground()).reshade(54));
            }
            Action::Send => {
                let mut scanner = args.scan();
                span.action = scanner.next_or(&["href", "xch_cmd"]).map(|action| {
                    if world.underline_hyperlinks {
                        span.flags.insert(TextStyle::Underline);
                    }
                    if world.use_custom_link_color {
                        span.foreground = Some(world.hyperlink_color.clone());
                    }
                    let sendto = if args.has_keyword(Keyword::Prompt) {
                        SendTo::Input
                    } else {
                        SendTo::World
                    };
                    Link::new(action, scanner.next_or(&["hint", "xch_hint"]), sendto)
                });
            }
            Action::Hyperlink => {
                let mut scanner = args.scan();
                span.action = scanner.next_or(&["href"]).map(|action| {
                    span.flags.insert(TextStyle::Underline);
                    if world.use_custom_link_color {
                        span.foreground = Some(world.hyperlink_color.clone());
                    }
                    Link::new(action, None, SendTo::Internet)
                });
            }
            Action::Font => {
                let mut scanner = args.scan();
                for fg in scanner
                    .next_or(&["color", "fgcolor"])
                    .unwrap_or("")
                    .split(",")
                {
                    match fg.to_lowercase().as_str() {
                        "blink" | "italic" => span.flags.insert(TextStyle::Italic),
                        "underline" => span.flags.insert(TextStyle::Underline),
                        "bold" => span.flags.insert(TextStyle::Bold),
                        "inverse" => span.flags.insert(TextStyle::Inverse),
                        color => {
                            if let Some(fg) = get_color(color) {
                                span.foreground = Some(fg.into_owned());
                            }
                        }
                    }
                }
                if let Some(bg) = scanner.next_or(&["back", "bgcolor"]) {
                    if let Some(bg) = get_color(bg) {
                        span.background = Some(bg.into_owned());
                    }
                }
            }
            Action::Version => {
                self.send(format!(
                    "\x1B[1zVERSION MXP=\"{}\" CLIENT={} VERSION=\"{}\" REGISTERED=YES>\n",
                    mxp::VERSION,
                    branding::APPNAME,
                    branding::VERSION
                ))
                .ok();
            }
            Action::Afk => {
                let mut scanner = args.scan();
                if world.send_mxp_afk_response {
                    let challenge = scanner.next_or(&["challenge"]).unwrap_or("");
                    self.send(format!(
                        "\x1B[1z<AFK {} {}>\n",
                        self.latest.input.elapsed().as_secs(),
                        challenge
                    ))
                    .ok();
                }
            }
            Action::Support => {
                self.send(Atom::supported(args)).ok();
            }
            Action::User => {
                if !world.player.is_empty() && world.connect_method == Some(AutoConnect::MXP) {
                    self.send(format!("{}\n", world.player)).ok();
                }
            }
            Action::Password => {
                if !world.password.is_empty() && world.connect_method == Some(AutoConnect::MXP) {
                    self.send(format!("{}\n", world.password)).ok();
                }
            }
            Action::P => {
                fragments.push(Fragment::Break);
                self.state.in_paragraph = true;
            }
            Action::Br => {
                fragments.push(Fragment::html("<br>"));
                span.foreground = Some(world.color(&WorldColor::WHITE).to_owned());
                span.background = Some(world.color(&WorldColor::BLACK).to_owned());
            }
            Action::Reset => {
                self.mxp_off(false);
            }
            // MXP options  (MXP OFF, MXP DEFAULT_OPEN, MXP DEFAULT_SECURE etc.
            Action::Mxp => {
                if args.has_keyword(Keyword::Off) {
                    self.mxp_off(true);
                }

                // TODO MUSHclient comments out everything below—why?
                if args.has_keyword(Keyword::DefaultLocked) {
                    self.state.mxp_mode_default = mxp::Mode::LOCKED;
                } else if args.has_keyword(Keyword::DefaultSecure) {
                    self.state.mxp_mode_default = mxp::Mode::SECURE;
                } else if args.has_keyword(Keyword::DefaultOpen) {
                    self.state.mxp_mode_default = mxp::Mode::OPEN;
                }

                if args.has_keyword(Keyword::IgnoreNewlines) {
                    self.state.in_paragraph = true;
                } else if args.has_keyword(Keyword::UseNewlines) {
                    self.state.in_paragraph = false;
                }
            }
            Action::Script => {
                self.state.mxp_script = true;
            }
            Action::Hr => {
                fragments.push(Fragment::html("<hr>"));
            }
            Action::Pre => {
                self.state.pre_mode = true;
            }
            Action::Ul => {
                span.list = Some(InList::Unordered);
            }
            Action::Ol => {
                span.list = Some(InList::Ordered(0));
            }
            Action::Li => {
                if let Some(list) = span.list.as_mut() {
                    fragments.push(Fragment::Break);
                    fragments.push(match list {
                        InList::Unordered => Fragment::text(" • "),
                        InList::Ordered(i) => {
                            *i += 1;
                            Fragment::text(format!(" {}. ", *i))
                        }
                    });
                }
            }
            Action::Img | Action::Image => {
                if let Some(xch_mode) = args.get("xch_mode") {
                    self.state.pueblo_active = true;
                    if xch_mode.eq_ignore_ascii_case("purehtml") {
                        self.state.suppress_newline = true;
                    } else if xch_mode.eq_ignore_ascii_case("html") {
                        self.state.suppress_newline = false;
                    }
                }
                if let Some(url) = args.get("url").or_else(|| args.get("src")) {
                    // TODO setting on MXP page to enable or disable images
                    fragments.push(Fragment::html(format!(
                        "<img src={}{}>",
                        url,
                        args.get("fname").unwrap_or(""),
                    )));
                }
            }
            Action::XchPage => {
                self.state.pueblo_active = true; // for correct newline processing
                self.mxp_off(false);
            }
            Action::Var => {
                let variable = args.get(0).unwrap_or("");
                if mxp::is_valid(variable) && !mxp::EntityMap::global(variable).is_some() {
                    span.variable = Some(variable.to_owned());
                }
            }
            _ => (),
        }
    }

    fn mxp_close_tags_from(&mut self, pos: usize) {
        if let Some(tag) = self.state.mxp_active_tags.get(pos) {
            self.style.truncate(tag.span_index);
            self.state.mxp_active_tags.truncate(pos);
        }
    }

    fn mxp_collected_entity(&mut self) -> Result<(), mxp::ParseError> {
        let bytestring = mem::take(&mut self.state.mxp_string);
        let text = str::from_utf8(&bytestring).map_err(|_| {
            mxp::ParseError::new(&format!("{:?}", bytestring), mxp::Error::MalformedBytes)
        })?;
        let name = text.trim();
        mxp::validate(name, mxp::Error::InvalidEntityName)?;
        if let Some(entity) = self.state.mxp_entities.get(text)? {
            let text = entity.as_bytes().to_vec();
            // if the entity happens to be < & > etc. don't reprocess it
            self.state.mxp_active = false;
            self.display_msg(text);
            self.state.mxp_active = true;
        }
        Ok(())
    }

    fn mxp_mode_change(&mut self, newmode: Option<mxp::Mode>) {
        let oldmode = self.state.mxp_mode;
        let newmode = newmode.unwrap_or(self.state.mxp_mode_default);
        let closing = oldmode.is_open() && !newmode.is_open();
        if closing {
            // don't close securely-opened tags here
            let closed = match self.state.mxp_active_tags.iter().rposition(|x| x.secure) {
                None => 0,
                Some(i) => i + 1,
            };
            self.mxp_close_tags_from(closed);
        }
        match newmode {
            mxp::Mode::OPEN | mxp::Mode::SECURE | mxp::Mode::LOCKED => {
                self.state.mxp_mode_default = mxp::Mode::OPEN
            }
            mxp::Mode::SECURE_ONCE => self.state.mxp_mode_previous = self.state.mxp_mode,
            mxp::Mode::PERM_OPEN | mxp::Mode::PERM_SECURE | mxp::Mode::PERM_LOCKED => {
                self.state.mxp_mode_default = newmode
            }
            _ => (),
        }
        self.state.mxp_mode = newmode;
    }

    pub fn send_window_sizes(&self, new_width: u16) {
        let [newhigh, newlow] = new_width.to_be_bytes();
        let height = unsafe { self.widget.height() / self.widget.font_metrics().height() } as u16;
        let [high, low] = height.to_be_bytes();
        // now tell them our size
        let packet = [
            telnet::IAC,
            telnet::SB,
            telnet::NAWS,
            newhigh,
            newlow,
            high,
            low,
            telnet::IAC,
            telnet::SE,
        ];
        self.send_packet(&packet);
    }

    // API methods

    pub fn send_packet(&self, data: &[u8]) {
        #[cfg(feature = "show-special")]
        self.append_to_notepad(
            Pad::PacketDebug,
            AlignmentFlag::AlignRight,
            telnet::escape(data),
        );
        if let Err(e) = self.send(data) {
            eprintln!("Error sending packet {:?}: {}", data, e);
        }
    }

    fn telnet_callbacks(&mut self, c: u8, verb: &str, confirm: &str) -> bool {
        let stop_on_true = enums![true];
        if self
            .plugins
            .send_to_all_until(Callback::TelnetRequest, (c, verb), stop_on_true)
        {
            self.plugins
                .send_to_all_until(Callback::TelnetRequest, (c, confirm), stop_on_true);
            true
        } else {
            false
        }
    }

    fn display_msg(&mut self, mut data: Vec<u8>) {
        #[cfg(debug_assertions)]
        println!("{}", String::from_utf8_lossy(&data));
        data = self
            .plugins
            .receive_from_all(Callback::PacketReceived, data);
        if data.len() == 0 {
            return; // plugin discarded it
        }
        self.style.clear_flags(); // MUD input cancels style flags

        let mut iter = data.iter_mut();

        #[cfg(feature = "show-special")]
        let mut old_phase = self.phase;

        while let Some(cref) = iter.next() {
            let c = *cref;

            #[cfg(feature = "show-special")]
            {
                if self.phase != old_phase {
                    self.flush();
                    self.cursor.insert_text_colored(
                        self.phase.to_str(),
                        Some(self.world.color(&WorldColor::BRIGHT_BLACK)),
                        None,
                    );
                    old_phase = self.phase;
                }
                if self.phase != Phase::Normal {
                    if let Some(escaped) = telnet::escape_char(c) {
                        self.append_to_notepad(Pad::PacketDebug, AlignmentFlag::AlignLeft, escaped)
                    } else if c.is_ascii() {
                        self.append_to_notepad(Pad::PacketDebug, AlignmentFlag::AlignLeft, &[c])
                    } else {
                        self.append_to_notepad(
                            Pad::PacketDebug,
                            AlignmentFlag::AlignLeft,
                            format!("{:#X}", c),
                        )
                    }
                }
            }

            // bail out of UTF-8 collection if a non-high order bit is found in the incoming stream
            if self.phase == Phase::Utf8Character && (c & 0x80) == 0 {
                self.output_bad_utf8();
            }
            // note that CR, LF, ESC and IAC can appear inside telnet negotiation now (version 4.48)
            if !(self.phase == Phase::Iac && c == telnet::IAC)
                && self.phase != Phase::Sb
                && self.phase != Phase::Subnegotiation
                && self.phase != Phase::SubnegotiationIac
                // the following characters will terminate any collection/negotiation phases
                //  newline, carriage-return, escape, IAC
                && b"\r\n\x1b\xff".contains(&c)
            {
                if self.phase == Phase::MxpRoomName
                    || self.phase == Phase::MxpRoomDescription
                    || self.phase == Phase::MxpRoomExits
                    || self.phase == Phase::MxpWelcome
                {
                    self.mxp_mode_change(None);
                }
                // cannot be in middle of escape sequence
                self.phase = Phase::Normal;
            }
            match self.phase {
                Phase::Esc => {
                    if c == b'[' {
                        self.phase = Phase::DoingCode;
                        self.state.ansi_code = 0;
                    } else {
                        self.phase = Phase::Normal;
                    }
                }
                Phase::Utf8Character => {
                    // append to our UTF8 sequence
                    self.state.utf8_sequence.push(c);

                    if let Ok(utf8array) = self.state.utf8_sequence.as_slice().try_into() {
                        match char::from_u32(u32::from_be_bytes(utf8array)) {
                            None => self.output_bad_utf8(),
                            Some(..) => {
                                self.phase = Phase::Normal;
                                self.bufoutput.append(&mut self.state.utf8_sequence);
                            }
                        }
                    }
                }
                Phase::DoingCode
                | Phase::Foreground256Start
                | Phase::Foreground256Finish
                | Phase::Background256Start
                | Phase::Background256Finish
                | Phase::Foreground24bFinish
                | Phase::Foreground24brFinish
                | Phase::Foreground24bgFinish
                | Phase::Foreground24bbFinish
                | Phase::Background24bFinish
                | Phase::Background24brFinish
                | Phase::Background24bgFinish
                | Phase::Background24bbFinish => {
                    self.flush(); // style is changing, so be sure to print whatever we've got
                    if c >= b'0' && c <= b'9' {
                        self.state.ansi_code = self.state.ansi_code * 10 + (c - b'0');
                    } else if c == b'm' {
                        self.interpret_code();
                        self.phase = Phase::Normal;
                    } else if c == b';' || c == b':' {
                        // separator, eg. ESC[ 38:5:<n>
                        self.interpret_code();
                        self.state.ansi_code = 0;
                    } else if c == b'z' {
                        // MXP line security mode
                        let mode = mxp::Mode(self.state.ansi_code);
                        if mode == mxp::Mode::RESET {
                            self.mxp_off(false);
                        } else {
                            self.mxp_mode_change(Some(mode));
                        }
                        self.phase = Phase::Normal;
                    } else {
                        self.phase = Phase::Normal;
                    }
                }
                Phase::Iac => {
                    if c == telnet::IAC {
                        break;
                    }
                    self.state.subnegotiation_type = 0; // no subnegotiation type yet
                    match c {
                        telnet::EOR | telnet::GA => {
                            self.phase = Phase::Normal;
                            self.state.last_line_with_iac_ga = self.state.linecount;
                            self.plugins.send_to_all(Callback::IacGa, ());
                            if self.world.convert_ga_to_newline {
                                *cref = b'\n';
                                break;
                            } else {
                                continue;
                            }
                        }
                        telnet::SB => self.phase = Phase::Sb,
                        telnet::WILL => self.phase = Phase::Will,
                        telnet::WONT => self.phase = Phase::Wont,
                        telnet::DO => self.phase = Phase::Do,
                        telnet::DONT => self.phase = Phase::Dont,
                        _ => self.phase = Phase::Normal,
                    }
                    continue;
                }
                // WILL - we have IAC WILL x - reply DO or DONT
                // (generally based on client option settings)
                // for unknown types we query plugins: function OnPluginTelnetRequest (num, type)
                //    eg. num = 200, type = WILL
                // They reply true or false to handle or not handle that telnet type
                Phase::Will => {
                    // telnet negotiation : in response to WILL, we say DONT
                    // (except for compression, MXP, TERMINAL_TYPE and SGA), we *will* handle that)
                    self.phase = Phase::Normal; // back to normal text after this character
                    let verb = match c {
                        telnet::COMPRESS | telnet::COMPRESS2 => {
                            if self.world.disable_compression {
                                telnet::DONT
                            } else {
                                if c == telnet::COMPRESS && self.state.supports_mccp_2 {
                                    // already agreed to MCCP 2 - no compression
                                    telnet::DONT
                                } else {
                                    if c == telnet::COMPRESS2 {
                                        self.state.supports_mccp_2 = true;
                                    }
                                    telnet::DO
                                }
                            }
                        }
                        telnet::SGA => telnet::DO, // Suppress GoAhead
                        telnet::MUD_SPECIFIC => telnet::DO,
                        telnet::ECHO => {
                            if self.world.no_echo_off {
                                telnet::DONT
                            } else {
                                self.state.no_echo = true;
                                telnet::DO
                            }
                        }
                        telnet::MXP => match self.world.use_mxp {
                            UseMxp::Never => telnet::DONT,
                            UseMxp::Query => {
                                self.mxp_on(false, false);
                                telnet::DO
                            }
                            _ => telnet::DO,
                        },
                        telnet::WILL_EOR => {
                            if self.world.convert_ga_to_newline {
                                telnet::DO
                            } else {
                                telnet::DONT
                            }
                        }
                        telnet::CHARSET => telnet::DO,
                        _ => {
                            if self.telnet_callbacks(c, "WILL", "SENT_DO") {
                                telnet::DO
                            } else {
                                telnet::DONT
                            }
                        }
                    };
                    self.send_packet(&[telnet::IAC, verb, c]);
                }
                // Received: IAC WONT x
                Phase::Wont => {
                    // telnet negotiation : in response to WONT, we say DONT
                    self.phase = Phase::Normal;
                    if !self.world.no_echo_off {
                        self.state.no_echo = false;
                    }
                    self.send_packet(&[telnet::IAC, telnet::DONT, c]);
                }
                // Received: IAC DO x
                // for unknown types we query plugins: function OnPluginTelnetRequest (num, type)
                //    eg. num = 200, type = DO
                // They reply true or false to handle or not handle that telnet type
                Phase::Do => {
                    // telnet negotiation : in response to DO, we say WILL for:
                    //  <102> (Aardwolf), SGA, echo, NAWS, CHARSET, MXP and Terminal type
                    // for others we query plugins to see if they want to handle it or not
                    // scoped borrow
                    self.phase = Phase::Normal;

                    let verb = match c {
                        // things we will do
                        telnet::SGA | telnet::MUD_SPECIFIC | telnet::ECHO | telnet::CHARSET => {
                            telnet::WILL
                        }
                        // for MTTS start back at sequence 0
                        telnet::TERMINAL_TYPE => {
                            self.state.ttype_sequence = 0;
                            telnet::WILL
                        }
                        telnet::NAWS => {
                            // option off - must be server initiated
                            if self.world.naws {
                                self.state.naws_wanted = true;
                                self.send_window_sizes(self.world.wrap_column);
                                telnet::WILL
                            } else {
                                telnet::WONT
                            }
                        }
                        telnet::MXP => match self.world.use_mxp {
                            UseMxp::Never => telnet::WONT,
                            UseMxp::Query => {
                                self.mxp_on(false, false);
                                telnet::WILL
                            }
                            _ => telnet::WILL,
                        },
                        _ => {
                            if self.telnet_callbacks(c, "DO", "SENT_WILL") {
                                telnet::WILL
                            } else {
                                telnet::WONT
                            }
                        }
                    };
                    self.send_packet(&[telnet::IAC, verb, c]);
                }
                // Received: IAC DONT x
                Phase::Dont => {
                    // telnet negotiation : in response to DONT, we say WONT
                    self.phase = Phase::Normal;
                    let mxp = self.state.mxp_active;
                    self.send_packet(&[telnet::IAC, telnet::WONT, c]);
                    match c {
                        telnet::MXP if mxp => self.mxp_off(true),
                        // for MTTS start back at sequence 0
                        telnet::TERMINAL_TYPE => self.state.ttype_sequence = 0,
                        _ => (),
                    }
                }
                // SUBNEGOTIATION - we have IAC SB c
                // remember c (the type) and start collecting the data, as in:
                // IAC SB c <data> IAC SE
                Phase::Sb => {
                    // note IAC SB COMPRESS is a special case because they forgot to specify
                    // the IAC SE, and thus we can't use normal negotiation
                    if c == telnet::COMPRESS {
                        self.phase = Phase::Compress;
                    } else {
                        self.state.subnegotiation_type = c;
                        self.state.subnegotiation_data.clear();
                        self.phase = Phase::Subnegotiation;
                    }
                }
                // SUBNEGOTIATION - we have IAC SB c (data)
                // if we get an IAC remember it, because it may or may not be followed by IAC or SE
                Phase::Subnegotiation => {
                    if c == telnet::IAC {
                        self.phase = Phase::SubnegotiationIac;
                    } else {
                        self.state.subnegotiation_data.push(c);
                    }
                }
                // COMPRESSION - we have IAC SB COMPRESS x
                Phase::Compress => {
                    self.phase = if c == telnet::WILL {
                        Phase::CompressWill // should get
                    } else {
                        Phase::Normal // error
                    };
                }
                // COMPRESSION - we have IAC SB COMPRESS IAC/WILL x   (MCCP v1)
                Phase::CompressWill => {
                    if c == telnet::SE {
                        // end of subnegotiation
                        self.state.mccp_ver = Some(Mccp::V1);
                        // special case, can't keep treating the  data as if it was not compressed
                        // skip SE (normaly done at end of loop)
                        iter.next();
                        // initialise compression library if not already done and copy
                        // compressed data to compression buffer
                        self.init_zlib(iter.into_slice().to_vec());
                        // done with this loop, now it needs to be decompressed
                        return;
                    } else {
                        self.phase = Phase::Normal; // error
                    }
                }

                // SUBNEGOTIATION - we have IAC SB x (data) IAC c
                // if the c after IAC is IAC then that becomes a single IAC (which we store now)
                // otherwise it should be SE, and we assume it is
                // otherwise we have an invalid sequence
                Phase::SubnegotiationIac => {
                    if c == telnet::IAC {
                        // have IAC SB x <data> IAC IAC
                        // store the single IAC
                        self.state.subnegotiation_data.push(c);
                        self.phase = Phase::Subnegotiation;
                    } else {
                        // see: http://www.gammon.com.au/forum/?id=10043
                        // we have to assume that anything other than IAC is a SE, because
                        // the spec is silent on what to do otherwise
                        // end of subnegotiation
                        // negotiation is over, next byte is plaintext
                        self.phase = Phase::Normal;
                        // subnegotiation is complete ...
                        // we have IAC SB <m_subnegotiation_type> <m_IAC_subnegotiation_data> IAC SE
                        match self.state.subnegotiation_type {
                            // turn MCCP v2 on
                            telnet::COMPRESS2 => {
                                if !self.world.disable_compression {
                                    self.state.mccp_ver = Some(Mccp::V2);
                                    // special case, can't keep treating the  data as if it was not compressed
                                    // skip SE (normaly done at end of loop)
                                    //iter.next();
                                    // initialise compression library if not already done and copy
                                    // compressed data to compression buffer
                                    self.init_zlib(iter.into_slice().to_vec());
                                    // done with this loop, now it needs to be decompressed
                                    return;
                                }
                            }
                            // turn MXP on, if required on subnegotiation
                            telnet::MXP => {
                                // if wanted now
                                if self.world.use_mxp == UseMxp::Command {
                                    self.mxp_on(false, false);
                                }
                            }
                            // terminal type request
                            telnet::TERMINAL_TYPE => {
                                if self.state.subnegotiation_data.get(0) == Some(&telnet::TTYPE_SEND)
                                {
                                    // we reply: IAC SB TERMINAL-TYPE IS ... IAC SE
                                    // see: RFC 930 and RFC 1060
                                    // also see: http://tintin.sourceforge.net/mtts/
                                    let p1 = [
                                        telnet::IAC,
                                        telnet::SB,
                                        telnet::TERMINAL_TYPE,
                                        telnet::TTYPE_IS,
                                    ];
                                    /*
                                    On the first TTYPE SEND request the client should return its name, preferably without a version number and in all caps.

                                    On the second TTYPE SEND request the client should return a terminal type, preferably in all caps.
                                      Console clients should report the name of the terminal emulator,
                                      other clients should report one of the four most generic terminal types.

                                        "DUMB"              Terminal has no ANSI color or VT100 support.
                                        "ANSI"              Terminal supports all ANSI color codes. Supporting blink and underline is optional.
                                        "VT100"             Terminal supports most VT100 codes, including ANSI color codes.
                                        "XTERM"             Terminal supports all VT100 and ANSI color codes, xterm 256 colors, mouse tracking, and the OSC color palette.

                                    If 256 color detection for non MTTS compliant servers is a must it's an option
                                      to report "ANSI-256COLOR", "VT100-256COLOR", or "XTERM-256COLOR".
                                      The terminal is expected to support VT100, mouse tracking, and the OSC color palette if "XTERM-256COLOR" is reported.

                                    On the third TTYPE SEND request the client should return MTTS followed by a bitvector. The bit values and their names are defined below.

                                            1 "ANSI"              Client supports all ANSI color codes. Supporting blink and underline is optional.
                                            2 "VT100"             Client supports most VT100 codes.
                                            4 "UTF-8"             Client is using UTF-8 character encoding.
                                            8 "256 COLORS"        Client supports all xterm 256 color codes.
                                           16 "MOUSE TRACKING"    Client supports xterm mouse tracking.
                                           32 "OSC COLOR PALETTE" Client supports the OSC color palette.
                                           64 "SCREEN READER"     Client is using a screen reader.
                                          128 "PROXY"             Client is a proxy allowing different users to connect from the same IP address.

                                    */
                                    let text = match self.state.ttype_sequence {
                                        0 => {
                                            self.state.ttype_sequence += 1;
                                            left(self.world.terminal_identification.as_bytes(), 20)
                                        }
                                        1 => {
                                            self.state.ttype_sequence += 1;
                                            b"ANSI"
                                        }
                                        _ if self.world.utf_8 => b"MTTS 13",
                                        _ => b"MTTS 9",
                                    };
                                    let p2 = [telnet::IAC, telnet::SE];
                                    let packet = [&p1, text, &p2].concat();
                                    self.send_packet(&packet);
                                }
                            }
                            // IAC SB CHARSET REQUEST DELIMITER <name> DELIMITER
                            /*

                            For backwards compatibility:

                            Server sends:  IAC DO CHARSET
                            Client sends:  IAC WILL CHARSET

                              or:

                            See: https://tools.ietf.org/html/rfc2066

                            Server sends:  IAC WILL CHARSET
                            Client sends:  IAC DO CHARSET

                            Server sends:  IAC SB CHARSET REQUEST DELIM NAME IAC SE
                            Client sends:  IAC SB CHARSET ACCEPTED NAME IAC SE
                            or
                            Client sends:  IAC SB CHARSET REJECTED IAC SE

                            where:

                              CHARSET: 0x2A
                              REQUEST: 0x01
                              ACCEPTED:0x02
                              REJECTED:0x03
                              DELIM:   some character that does not appear in the charset name, other than IAC, eg. comma, space
                              NAME:    the character string "UTF-8" (or some other name like "S-JIS")

                            */
                            telnet::CHARSET => {
                                // must have at least REQUEST DELIM NAME [ DELIM NAME2 ...]
                                let data = self.state.subnegotiation_data.clone();
                                if data.len() >= 3 && data[0] == 1 {
                                    let delim = data[1];
                                    let charset: &[u8] = if self.world.utf_8 {
                                        // hack! ugh.
                                        b"UTF-8"
                                    } else {
                                        b"US-ASCII"
                                    };
                                    let mut found = false;
                                    for fragment in data[2..].split(|&c| c == delim) {
                                        if fragment == charset {
                                            found = true;
                                            let p1 = [
                                                telnet::IAC,
                                                telnet::SB,
                                                telnet::CHARSET,
                                                telnet::ACCEPT,
                                            ];
                                            let p2 = [telnet::IAC, telnet::SE];
                                            let packet = [&p1, left(fragment, 20), &p2].concat();
                                            self.send_packet(&packet);
                                        }
                                    }
                                    if !found {
                                        let packet = [
                                            telnet::IAC,
                                            telnet::SB,
                                            telnet::REJECT,
                                            telnet::IAC,
                                            telnet::SE,
                                        ];
                                        self.send_packet(&packet);
                                    }
                                }
                            }
                            telnet::MUD_SPECIFIC => {
                                let data = String::from_utf8_lossy(&self.state.subnegotiation_data)
                                    .into_owned();
                                self.plugins.send_to_all(Callback::TelnetOption, data);
                            }
                            _ => {
                                let sbtype = self.state.subnegotiation_type;
                                let data = String::from_utf8_lossy(&self.state.subnegotiation_data)
                                    .into_owned();
                                self.plugins
                                    .send_to_all(Callback::TelnetSubnegotiation, (sbtype, data));
                            }
                        }
                    }
                }
                Phase::MxpElement => match c {
                    b'>' => {
                        if let Err(e) = self.mxp_collected_element() {
                            self.handle_mxp_error(e);
                        }
                        self.phase = Phase::Normal;
                    }
                    b'<' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedElement,
                        ));
                        self.state.mxp_string.clear();
                    }
                    b'\'' | b'"' => {
                        self.state.mxp_string.push(c);
                        self.state.mxp_quote_terminator = Some(c);
                        self.phase = Phase::MxpQuote;
                    }
                    b'-' => {
                        self.state.mxp_string.push(c);
                        if self.state.mxp_string.starts_with(b"!--") {
                            self.phase = Phase::MxpComment;
                        }
                    }
                    _ => self.state.mxp_string.push(c),
                },
                Phase::MxpComment => {
                    match c {
                        b'>' if self.state.mxp_string.ends_with(b"--") => {
                            // discard comment
                            self.phase = Phase::Normal;
                        }
                        _ => self.state.mxp_string.push(c),
                    }
                }
                Phase::MxpQuote => {
                    if self.state.mxp_quote_terminator == Some(c) {
                        self.phase = Phase::MxpElement;
                        self.state.mxp_quote_terminator = None;
                    }
                    self.state.mxp_string.push(c);
                }
                Phase::MxpEntity => match c {
                    b';' => {
                        self.phase = Phase::Normal;
                        if let Err(e) = self.mxp_collected_entity() {
                            self.handle_mxp_error(e);
                        }
                    }
                    b'&' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedEntity,
                        ));
                        self.state.mxp_string.clear();
                    }
                    b'<' => {
                        self.state.mxp_string.push(c);
                        self.handle_mxp_error(mxp::ParseError::new(
                            &String::from_utf8_lossy(&self.state.mxp_string),
                            mxp::Error::UnterminatedEntity,
                        ));
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpElement;
                    }
                    _ => self.state.mxp_string.push(c),
                },
                Phase::MxpRoomName
                | Phase::MxpRoomDescription
                | Phase::MxpRoomExits
                | Phase::MxpWelcome => {
                    // nope
                }
                Phase::Normal => match c {
                    telnet::ESC => self.phase = Phase::Esc,
                    telnet::IAC => {
                        if self.phase == Phase::Iac {
                            self.bufoutput.push(c);
                            self.phase = Phase::Normal;
                        } else {
                            self.phase = Phase::Iac;
                        }
                    }
                    b'<' if self.state.mxp_active && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpElement;
                    }
                    b'&' if self.state.mxp_active && self.state.mxp_mode.is_mxp() => {
                        self.state.mxp_string.clear();
                        self.phase = Phase::MxpEntity;
                    }
                    _ => self.bufoutput.push(c),
                },
            }
        }
        self.flush();
    }
}
