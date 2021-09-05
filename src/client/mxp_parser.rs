use std::iter::Iterator;
use std::{io, mem, str};

use cpp_core::CppBox;
use qt_core::QString;

use super::Client;
use crate::binding::text::CharFormat;
use crate::binding::Printable;
use crate::client::color::{Colors, WorldColor};
use crate::client::state::Phase;
use crate::client::style::TextStyle;
use crate::constants::branding;
use crate::mxp;
use crate::script::Callback;
use crate::world::AutoConnect;

#[derive(Debug)]
enum Fragment {
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

#[derive(Debug, Error)]
pub enum ParseError {
    Mxp(mxp::ParseError),
    Io(io::Error),
}

impl Client {
    pub(super) fn handle_mxp_error(&self, err: mxp::ParseError) {
        eprintln!("MXP Error: {}", err);
    }

    pub(super) fn handle_mxp_io_error(&self, err: ParseError) -> io::Result<()> {
        match err {
            ParseError::Mxp(e) => {
                self.handle_mxp_error(e);
                Ok(())
            }
            ParseError::Io(e) => Err(e),
        }
    }

    fn mxp_restore_mode(&mut self) {
        if self.state.mxp_mode == mxp::Mode::SECURE_ONCE {
            self.state.mxp_mode = self.state.mxp_mode_previous;
        }
    }

    pub(super) fn mxp_off(&mut self, completely: bool) {
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

    pub(super) fn mxp_on(&mut self, pueblo: bool, manual: bool) {
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

    fn mxp_findtag(&self, secure: bool, name: &str) -> Result<(usize, &mxp::Tag), mxp::ParseError> {
        for (i, tag) in self.state.mxp_active_tags.iter().enumerate().rev() {
            if tag.name.eq_ignore_ascii_case(name) {
                if !secure && tag.secure {
                    return Err(mxp::ParseError::new(
                        name,
                        mxp::Error::TagOpenedInSecureMode,
                    ));
                } else {
                    return Ok((i, tag));
                }
            }
            if !secure && tag.secure {
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

        let (closed, tag) = self.mxp_findtag(was_secure, name)?;
        if let Some(template) = &tag.anchor_template {
            let select = self.cursor.document().select(tag.text_index..);
            let fmt = CharFormat::new();
            let text = select.text();
            let anchor = template.replace("&text;", &text);
            fmt.set_anchor_href(&anchor);
            select.merge_char_format(&fmt);
        }
        self.mxp_close_tags_from(closed);
        Ok(())
    }

    fn mxp_definition(&mut self, tag: &str) -> Result<(), mxp::ParseError> {
        let was_secure = self.state.mxp_mode.is_secure();
        self.mxp_restore_mode();
        if !was_secure {
            return Err(mxp::ParseError::new(
                tag,
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

    pub(super) fn mxp_collected_element(&mut self) -> Result<(), ParseError> {
        let tag =
            *self.state.mxp_string.get(0).ok_or_else(|| {
                mxp::ParseError::new("collected element", mxp::Error::EmptyElement)
            })?;
        let bytestring = mem::take(&mut self.state.mxp_string);
        let text = str::from_utf8(&bytestring).map_err(|_| {
            mxp::ParseError::new(&format!("{:?}", bytestring), mxp::Error::MalformedBytes)
        })?;

        match tag {
            b'!' => self.mxp_definition(&text[1..]).map_err(Into::into),
            b'/' => self.mxp_endtag(&text[1..]).map_err(Into::into),
            _ => self.mxp_start_tag(text),
        }
    }

    fn mxp_start_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        self.flush()?; // probably going to change style or insert HTML
        let secure = self.state.mxp_mode.is_secure();
        self.mxp_restore_mode();
        let mut words = mxp::Words::new(tag);
        let name = words.validate_next_or(mxp::Error::InvalidElementName)?;
        let component = self.state.mxp_elements.get_component(name)?;
        let flags = component.flags();
        self.state.mxp_active_tags.push(mxp::Tag {
            name: name.to_owned(),
            secure,
            no_reset: flags.contains(mxp::TagFlag::NoReset),
            span_index: self.style.len(),
            text_index: self.cursor.position(),
            anchor_template: None,
        });
        if !flags.contains(mxp::TagFlag::Open) && !secure {
            return Err(mxp::ParseError::new(name, mxp::Error::ElementWhenNotSecure).into());
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
                self.mxp_open_atom(&mut span, &mut fragments, atom.action, args)?;
            }
            mxp::ElementComponent::Custom(el) => {
                // create a temporary vector to avoid borrow conflict
                // could clone the element instead, but that seems like a waste
                let actions: Result<Vec<_>, mxp::ParseError> = el
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
                    self.mxp_open_atom(&mut span, &mut fragments, action, newargs)?;
                }
            }
        }

        if span != unchanged {
            self.style.push(span);
        }

        for fragment in fragments {
            match fragment {
                Fragment::Html(text) => {
                    self.flush()?;
                    self.insert_html(text);
                }
                Fragment::Text(text) => {
                    self.flush()?;
                    self.insert_text(text);
                }
                Fragment::Break => {
                    self.insert_line();
                }
            }
        }

        Ok(())
    }

    fn mxp_open_atom(
        &mut self,
        span: &mut mxp::Span,
        fragments: &mut Vec<Fragment>,
        mut action: mxp::Action,
        args: mxp::Arguments,
    ) -> io::Result<()> {
        use mxp::{Action, Atom, InList, Keyword, Link, SendTo};
        const SPECIAL_LINK: &str = "&text;";
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
        if action == Action::Hyperlink && args.get("xch_cmd").is_some() {
            self.state.pueblo_active = true; // for correct newline processing
            action = Action::Send;
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
                let action = scanner
                    .next_or(&["href", "xch_cmd"])
                    .unwrap_or(SPECIAL_LINK);
                if world.underline_hyperlinks {
                    span.flags.insert(TextStyle::Underline);
                }
                if world.use_custom_link_color {
                    span.foreground = Some(world.hyperlink_color.clone());
                }
                let hint = scanner.next_or(&["hint", "xch_hint"]);
                let sendto = if args.has_keyword(Keyword::Prompt) {
                    SendTo::Input
                } else {
                    SendTo::World
                };
                span.action = Some(Link::new(action, hint, sendto));
                if action.contains(SPECIAL_LINK) {
                    if let Some(mut tag) = self.state.mxp_active_tags.last_mut() {
                        let template = if args.has_keyword(Keyword::Prompt) {
                            ["echo:", action].concat()
                        } else {
                            ["send:", action].concat()
                        };
                        tag.anchor_template = Some(template);
                    }
                }
            }
            Action::Hyperlink => {
                let mut scanner = args.scan();
                let action = scanner.next_or(&["href"]).unwrap_or(SPECIAL_LINK);
                span.flags.insert(TextStyle::Underline);
                if world.use_custom_link_color {
                    span.foreground = Some(world.hyperlink_color.clone());
                }
                span.action = Some(Link::new(action, None, SendTo::Internet));
                if action.contains(SPECIAL_LINK) {
                    if let Some(mut tag) = self.state.mxp_active_tags.last_mut() {
                        tag.anchor_template = Some(action.to_owned());
                    }
                }
            }
            Action::Font => {
                let mut scanner = args.scan();
                for fg in scanner
                    .next_or(&["color", "fgcolor"])
                    .unwrap_or("")
                    .split(',')
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
                ))?;
            }
            Action::Afk => {
                let mut scanner = args.scan();
                if world.send_mxp_afk_response {
                    let challenge = scanner.next_or(&["challenge"]).unwrap_or("");
                    self.send(format!(
                        "\x1B[1z<AFK {} {}>\n",
                        self.latest.input.elapsed().as_secs(),
                        challenge
                    ))?;
                }
            }
            Action::Support => {
                self.send(Atom::supported(args))?;
            }
            Action::User => {
                if !world.player.is_empty() && world.connect_method == Some(AutoConnect::Mxp) {
                    self.send(format!("{}\n", self.world.player))?;
                }
            }
            Action::Password => {
                if !world.password.is_empty() && world.connect_method == Some(AutoConnect::Mxp) {
                    self.send(format!("{}\n", self.world.password))?;
                }
            }
            Action::P => {
                fragments.push(Fragment::Break);
                self.state.in_paragraph = true;
            }
            Action::Br => {
                fragments.push(Fragment::Break);
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
                if mxp::is_valid(variable) && mxp::EntityMap::global(variable).is_none() {
                    span.variable = Some(variable.to_owned());
                }
            }
            _ => (),
        }
        Ok(())
    }

    fn mxp_close_tags_from(&mut self, pos: usize) {
        if let Some(tag) = self.state.mxp_active_tags.get(pos) {
            self.style.truncate(tag.span_index);
            self.state.mxp_active_tags.truncate(pos);
        }
    }

    pub(super) fn mxp_collected_entity(&mut self) -> Result<(), ParseError> {
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
            self.display_msg(text)?;
            self.state.mxp_active = true;
        }
        Ok(())
    }

    pub(super) fn mxp_mode_change(&mut self, newmode: Option<mxp::Mode>) {
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
}
