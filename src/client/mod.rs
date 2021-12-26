use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::iter::Iterator;
use std::os::raw::c_double;
use std::rc::Rc;
use std::time::Instant;
use std::{mem, str};

use cpp_core::{CastFrom, Ptr};
use enumeration::Enum;
#[cfg(feature = "show-special")]
use qt_core::AlignmentFlag;
use qt_core::{QBox, QPtr};
use qt_gui::q_text_cursor::{MoveOperation, SelectionType};
use qt_network::q_abstract_socket::SocketState;
use qt_network::QTcpSocket;
use qt_widgets::q_message_box::Icon;
use qt_widgets::{QLineEdit, QTextBrowser, QWidget};

use crate::api::Api;
use crate::binding::color::Colored;
use crate::binding::text::{CharFormat, Cursor};
use crate::binding::{Printable, RColor, RIODevice, RWidget};
use crate::client::state::Latest;
use crate::constants::config;
use crate::escape::{ansi, telnet};
use crate::mxp;
use crate::script::{Callback, PluginHandler, Senders};
use crate::tr::TrContext;
use crate::ui::Notepad;
#[cfg(feature = "show-special")]
use crate::ui::Pad;
use crate::world::{LogFormat, LogMode, World};

pub mod color;
pub mod state;
mod stream;
use stream::MudStream;
pub mod style;

mod display_msg;
mod mxp_parser;

use color::WorldColor;
use state::{ClientState, Phase};
use style::{Style, TextStyle};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum Log {
    Output,
    Input,
    Script,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct LogConfig<'a> {
    enabled: bool,
    prefix: &'a str,
    suffix: &'a str,
}

impl<'a> LogConfig<'a> {
    fn get(kind: Log, world: &'a World) -> Self {
        match kind {
            Log::Output => LogConfig {
                enabled: world.log_output,
                prefix: &world.log_preamble_output,
                suffix: &world.log_postamble_output,
            },
            Log::Input => LogConfig {
                enabled: world.log_input,
                prefix: &world.log_preamble_input,
                suffix: &world.log_postamble_input,
            },
            Log::Script => LogConfig {
                enabled: world.log_notes,
                prefix: &world.log_preamble_notes,
                suffix: &world.log_postamble_notes,
            },
        }
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
    line: Vec<u8>,
    pub plugins: PluginHandler,
    world: Rc<World>,
    notepad: Rc<RefCell<Notepad>>,
    phase: Phase,
    style: Style,
    state: ClientState,
    latest: Latest,
    log: Option<BufWriter<File>>,
}

impl RWidget for Client {
    fn widget(&self) -> Ptr<QWidget> {
        // SAFETY: `widget` is valid.
        unsafe { Ptr::cast_from(&self.widget) }
    }
}

impl Client {
    /// # Safety
    ///
    /// `output` and `input` must be valid and non-null.
    pub unsafe fn new(
        output: QPtr<QTextBrowser>,
        input: QPtr<QLineEdit>,
        socket: QBox<QTcpSocket>,
        world: Rc<World>,
    ) -> Self {
        let notepad = Rc::new(RefCell::new(Notepad::new(&world.name)));
        let socket = RIODevice::new(socket);
        // SAFETY: all fields are valid.
        let api = unsafe {
            Api::new(
                output.clone(),
                input,
                socket.clone(),
                world.clone(),
                notepad.clone(),
                Rc::new(RefCell::new(Senders::new())),
            )
        };

        // SAFETY: `output` is valid.
        let cursor = unsafe { Cursor::get(&output) };
        let charfmt = cursor.format.text.clone();
        let mut this = Self {
            notepad,
            cursor,
            stream: MudStream::new(socket.clone()),
            socket,
            bufinput: [0; config::SOCKET_BUFFER],
            bufoutput: Vec::new(),
            line: Vec::new(),
            style: Style::new(charfmt, world.clone()),
            widget: output,
            world,
            phase: Phase::Normal,
            state: ClientState::new(),
            plugins: PluginHandler::new(api),
            latest: Latest::new(),
            log: None,
        };
        this.open_log();
        this.load_plugins();
        this
    }

    fn load_plugins(&mut self) {
        for path in &self.world.plugins {
            if let Err(e) = self.plugins.load_plugin_file(path) {
                self.alert(
                    Icon::Warning,
                    tr!("Couldn't load plugin at {}", path.to_string_lossy()),
                    &e,
                );
            }
        }
        if let Err(e) = self.plugins.load_plugin(self.world.world_plugin()) {
            self.alert(Icon::Warning, tr!("Couldn't compile world script"), &e);
        }
        self.plugins.sort();
    }

    fn open_log(&mut self) {
        self.log = None;
        let world = &*self.world;
        if let Some(file) = &world.auto_log_file_name {
            match fs::OpenOptions::new()
                .write(true)
                .truncate(world.log_mode == LogMode::Overwrite)
                .create(true)
                .open(file)
            {
                Err(e) => self.alert(Icon::Warning, tr!("Unable to open log file"), &e),
                Ok(..) if world.log_format == LogFormat::Html => self.log = None,
                Ok(file) => self.log = Some(BufWriter::with_capacity(config::LOG_BUFFER, file)),
            }
        }
    }

    pub fn on_save(&mut self) {
        self.plugins.alter_userdata(|api| api.save_variables());
    }

    pub fn set_world(&mut self, world: Rc<World>) {
        let reload_log = self.world.auto_log_file_name != world.auto_log_file_name;
        let reload_plugins = self.world.plugins != world.plugins;
        self.style.set_world(world.clone());
        if !reload_plugins {
            if let Err(e) = self.plugins.update_world_plugin(&self.world, &world) {
                self.alert(Icon::Warning, tr!("Couldn't compile world script"), &e);
            }
            self.plugins
                .alter_userdata(|api| api.set_world(world.clone()));
        }
        self.world = world;
        if reload_plugins {
            self.plugins.clear();
            self.load_plugins();
        }
        if reload_log {
            self.open_log();
        }
    }

    pub fn set_spacing(&mut self, spacing: c_double) {
        self.cursor.format.block.set_line_height(spacing);
        self.plugins.alter_userdata(|api| api.set_spacing(spacing));
    }

    pub fn retitle(&mut self, name: &str) {
        self.notepad.borrow_mut().retitle(name);
    }

    pub fn connect(&mut self) {
        if self.socket.state() == SocketState::UnconnectedState {
            self.socket.connect(&self.world.site, self.world.port);
            self.latest.connected = Instant::now();
        }
    }

    pub fn disconnect(&mut self) {
        if self.socket.state() != SocketState::UnconnectedState {
            // don't want reconnect on manual disconnect
            self.state.disconnect_ok = true;
            // work out how long they were connected
            self.state.total_connect_duration += self.latest.connected.elapsed();
            self.mxp_off(true);
            self.socket.close();
            self.stream.reset();
        }
    }

    fn send<S: AsRef<[u8]>>(&mut self, buf: S) -> io::Result<()> {
        let buf = buf.as_ref();
        if self.world.log_format == LogFormat::Raw {
            self.write_to_log(Log::Input, buf);
        }
        self.socket.write_all(buf)
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
            self.scroll_to_bottom();
        }
        let matched_alias = self.plugins.alias(&command)?;
        command.push('\n');
        if world.log_format == LogFormat::Text {
            self.write_to_log(Log::Input, command.as_bytes());
        }
        if self.bufoutput.is_empty() {
            self.bufoutput.push(b'\n');
        }
        if matched_alias {
            Ok(())
        } else {
            self.send(&command)
        }
    }

    pub fn read(&mut self) -> io::Result<()> {
        let res = self.stream.read(&mut self.bufinput)?;
        if res > 0 {
            self.display_msg(self.bufinput[0..res].to_vec())
        } else {
            Ok(())
        }
    }

    fn scroll_to_bottom(&self) {
        unsafe {
            let scrollbar = self.widget.vertical_scroll_bar();
            scrollbar.set_value(scrollbar.maximum());
        }
    }

    fn insert_line(&mut self) {
        self.bufoutput.push(b'\n');
    }

    fn insert_html<S: Printable>(&self, text: S) {
        self.cursor.insert_html(text);
    }

    fn insert_text<S: Printable>(&self, text: S) {
        self.cursor.insert_text_formatted(text, self.style.format());
    }

    pub fn print<S: Printable>(&self, text: S) {
        self.insert_text(text);
    }

    pub fn println<S: Printable>(&self, text: S) {
        self.cursor.insert_block();
        self.insert_text(text);
        self.scroll_to_bottom();
    }

    #[cfg(feature = "show-special")]
    pub fn append_to_notepad<S: Printable>(&self, kind: Pad, align: AlignmentFlag, text: S) {
        self.notepad.borrow_mut().append_aligned(kind, align, text);
    }

    fn handle_line(&mut self, line: Option<&[u8]>) -> io::Result<()> {
        let line = line.unwrap_or(&self.line);
        let s = String::from_utf8_lossy(line);
        self.plugins.send_to_all(Callback::LineReceived, line);
        let requests = self.plugins.trigger(&s)?;

        if requests.hide {
            self.cursor.select(SelectionType::BlockUnderCursor).delete();
            return Ok(());
        }
        let format = CharFormat::new();
        if let Some(fg) = requests.foreground {
            format.set_foreground_color(&fg);
        }
        if let Some(bg) = requests.background {
            format.set_background_color(&bg);
        }
        if requests.make_bold {
            format.set_bold(true);
        }
        if requests.make_italic {
            format.set_italic(true);
        }
        if requests.make_underline {
            format.set_underline(true);
        }
        self.cursor
            .select(SelectionType::BlockUnderCursor)
            .merge_char_format(&format);
        Ok(())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.bufoutput.is_empty() {
            return Ok(());
        }
        if self.bufoutput == b"\n" {
            if !self.line.is_empty() {
                self.handle_line(None)?;
                self.line.clear();
            }
            return Ok(());
        }
        let trailing_newline = self.bufoutput.len() > 1 && self.bufoutput.ends_with(b"\n");

        if trailing_newline {
            self.bufoutput.pop();
        }
        let has_newline = trailing_newline || self.bufoutput.contains(&b'\n');
        let pos = self.cursor.position();
        self.print(&self.bufoutput);
        self.line.append(&mut self.bufoutput);

        if has_newline {
            let mut lines = if trailing_newline {
                Vec::new()
            } else {
                self.line
                    .split_off(self.line.iter().rposition(|&c| c == b'\n').unwrap() + 1)
            };
            mem::swap(&mut lines, &mut self.line);
            self.cursor.set_position(pos);
            for line in lines.split(|&c| c == b'\n') {
                if !line.is_empty() {
                    self.handle_line(Some(line))?;
                }
                self.cursor.move_position(MoveOperation::NextBlock, 1);
            }
            self.cursor.move_position(MoveOperation::End, 1);
        }
        if trailing_newline {
            self.bufoutput.push(b'\n');
        }
        Ok(())
    }

    fn write_to_log(&mut self, kind: Log, buf: &[u8]) {
        fn try_write(log: &mut BufWriter<File>, cfg: LogConfig, buf: &[u8]) -> io::Result<()> {
            if !cfg.prefix.is_empty() {
                log.write_all(cfg.prefix.as_bytes())?;
            }
            log.write_all(buf)?;
            if !cfg.suffix.is_empty() {
                log.write_all(cfg.suffix.as_bytes())?;
            }
            #[cfg(debug_assertions)]
            log.flush()?; // realtime logging
            Ok(())
        }
        if let Some(log) = self.log.as_mut() {
            let cfg = LogConfig::get(kind, &self.world);
            if !cfg.enabled {
                return;
            }
            if let Err(e) = try_write(log, cfg, buf) {
                self.alert(Icon::Warning, "Couldn't write to log", &e);
                self.log = None;
            }
        }
    }

    fn output_bad_utf8(&mut self) {
        self.bufoutput.append(&mut self.state.utf8_sequence);
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

    pub fn send_window_sizes(&mut self, new_width: u16) -> io::Result<()> {
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
        self.send_packet(&packet)
    }

    // API methods

    pub fn send_packet(&mut self, data: &[u8]) -> io::Result<()> {
        #[cfg(feature = "show-special")]
        self.append_to_notepad(
            Pad::PacketDebug,
            AlignmentFlag::AlignRight,
            telnet::escape(data),
        );
        self.send(data)
    }

    fn start_decompressing(&mut self, left: Vec<u8>, data: Vec<u8>) {
        if self.world.log_format == LogFormat::Raw {
            self.write_to_log(Log::Output, &data[..data.len() - left.len()]);
        }
        self.stream.start_decompressing(left);
    }
}
