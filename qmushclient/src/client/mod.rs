use std::cell::RefCell;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::iter::Iterator;
use std::os::raw::c_double;
use std::rc::Rc;
use std::time::Instant;
use std::{mem, str};

use enumeration::Enum;
use qmushclient_scripting::{Callback, PluginHandler, Senders};
#[cfg(feature = "show-special")]
use qmushlient_scripting::Pad;
#[cfg(feature = "show-special")]
use qt::core::AlignmentFlag;
use qt::gui::{MoveOperation, QColor, QTextCharFormat, QTextCursor, SelectionType};
use qt::network::{QTcpSocket, SocketState};
use qt::traits::{Colored, Printable, Widget};
use qt::widgets::{MessageBoxIcon, QLineEdit, QTextBrowser};
use tr::TrContext;

use crate::api::{Api, ApiState, Notepad};
use crate::client::state::Latest;
use crate::constants::{config, Paths};
use crate::escape::{ansi, telnet};
use crate::mxp;
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
pub struct Client<P: PluginHandler> {
    widget: QTextBrowser,
    cursor: QTextCursor,
    socket: QTcpSocket,
    stream: MudStream,
    bufinput: [u8; config::SOCKET_BUFFER],
    bufoutput: Vec<u8>,
    line: Vec<u8>,
    pub plugins: P,
    world: Rc<World>,
    notepad: Rc<RefCell<Notepad>>,
    phase: Phase,
    style: Style,
    state: ClientState,
    api_state: Rc<ApiState>,
    latest: Latest,
    log: Option<BufWriter<File>>,
}

impl<P: PluginHandler<Userdata = Api>> Client<P> {
    /// # Safety
    ///
    /// `output` and `input` must be valid and non-null.
    pub fn new(
        output: QTextBrowser,
        input: QLineEdit,
        socket: QTcpSocket,
        world: Rc<World>,
        paths: &'static Paths,
    ) -> Self {
        let notepad = Rc::new(RefCell::new(Notepad::new(&world.name)));
        let api_state = Rc::new(ApiState::default());
        // SAFETY: all fields are valid.
        let api = Api::new(
            output.clone(),
            input,
            socket.clone(),
            world.clone(),
            api_state.clone(),
            notepad.clone(),
            Rc::new(RefCell::new(Senders::new())),
            paths,
        );

        let cursor = output.text_cursor();
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
            api_state,
            plugins: P::new(api),
            latest: Latest::new(),
            log: None,
        };
        this.open_log();
        this.load_plugins();
        this
    }

    pub fn on_save(&mut self) {
        self.plugins.alter_userdata(|api| api.save_variables());
    }

    pub fn set_spacing(&mut self, spacing: c_double) {
        self.cursor.format.block.set_line_height(spacing);
        self.plugins.alter_userdata(|api| api.set_spacing(spacing));
    }
}

impl<P: PluginHandler<Userdata = Api, PluginWorld = World>> Client<P> {
    pub fn set_world(&mut self, world: Rc<World>) {
        let reload_log = self.world.auto_log_file_name != world.auto_log_file_name;
        let reload_plugins = self.world.plugins != world.plugins;
        self.style.set_world(world.clone());
        if !reload_plugins {
            if let Err(e) = self.plugins.update_world_plugin(&self.world, &world) {
                self.warn(tr!("Couldn't compile world script"), &e);
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
}

impl<P: PluginHandler> Client<P> {
    fn warn<S: Printable, E: std::error::Error + ?Sized>(&self, text: S, err: &E) {
        self.widget.alert(MessageBoxIcon::Warning, text, err)
    }

    fn load_plugins(&mut self) {
        for path in &self.world.plugins {
            if let Err(e) = self.plugins.load_plugin_file(path) {
                self.warn(
                    tr!("Couldn't load plugin at {}", path.to_string_lossy()),
                    &e,
                );
            }
        }
        if let Err(e) = self.plugins.load_plugin(self.world.world_plugin()) {
            self.warn(tr!("Couldn't compile world script"), &e);
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
                Err(e) => self.warn(tr!("Unable to open log file"), &e),
                Ok(..) if world.log_format == LogFormat::Html => self.log = None,
                Ok(file) => self.log = Some(BufWriter::with_capacity(config::LOG_BUFFER, file)),
            }
        }
    }

    pub fn retitle(&mut self, name: &str) {
        self.notepad.borrow_mut().set_name(name);
    }

    pub fn connect(&mut self) {
        if self.socket.state() == SocketState::UnconnectedState {
            self.socket.connect(&self.world.site, self.world.port);
            self.latest.connected = Instant::now();
        }
    }

    pub fn disconnect(&mut self) {
        if self.socket.state() != SocketState::UnconnectedState {
            self.api_state.disconnect_ok.set(true);
            let connect_duration = self.api_state.total_connect_duration.get();
            self.api_state
                .total_connect_duration
                .set(connect_duration + self.latest.connected.elapsed());
            self.mxp_off(true);
            self.socket.close();
            self.stream.reset();
            self.api_state.compressing.set(false);
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
            if !world.keep_commands_on_same_line && !self.cursor.is_at_block_start() {
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
        loop {
            let res = self.stream.read(&mut self.bufinput)?;
            if res == 0 {
                return Ok(());
            }
            self.display_msg(self.bufinput[0..res].to_vec())?;
            // avoids a weird interaction between Qt and Decompress
            if res != self.bufinput.len() {
                return Ok(());
            }
        }
    }

    fn scroll_to_bottom(&self) {
        let scrollbar = self.widget.vertical_scroll_bar();
        scrollbar.set_value(scrollbar.maximum());
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
        let format = QTextCharFormat::new();
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
            log.flush()?;
            Ok(())
        }
        if let Some(log) = self.log.as_mut() {
            let cfg = LogConfig::get(kind, &self.world);
            if !cfg.enabled {
                return;
            }
            if let Err(e) = try_write(log, cfg, buf) {
                self.warn("Couldn't write to log", &e);
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
    fn interpret_256_ansi(&mut self, code: u8) {
        fn build_ansi_color(state: &ClientState) -> WorldColor {
            WorldColor::Plain(QColor::rgb(
                state.ansi_red,
                state.ansi_green,
                state.ansi_blue,
            ))
        }
        match self.phase {
            Phase::Foreground256Start => match code {
                5 => {
                    self.state.ansi_code = 0;
                    self.phase = Phase::Foreground256Finish;
                }
                2 => {
                    self.state.ansi_code = 0;
                    self.phase = Phase::Foreground24bFinish;
                }
                _ => self.phase = Phase::Normal,
            },
            Phase::Background256Start => match code {
                5 => {
                    self.state.ansi_code = 0;
                    self.phase = Phase::Background256Finish;
                }
                2 => {
                    self.state.ansi_code = 0;
                    self.phase = Phase::Background24bFinish;
                }
                _ => self.phase = Phase::Normal,
            },
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
        let height = (self.widget.height() / self.widget.font_metrics().height()) as u16;
        let [high, low] = height.to_be_bytes();
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
        self.api_state.compressing.set(true);
    }
}
