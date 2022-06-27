use std::env::consts::OS;
use std::net::SocketAddr;
use std::os::raw::c_int;
use std::str;
use std::time::SystemTime;

use libsqlite3_sys::{SQLITE_VERSION, SQLITE_VERSION_NUMBER};
use mlua::{Lua, Result, Value};
use qt_core::{GlobalColor, KeyboardModifier, MouseButton};
use qt_gui::q_font::StyleHint;
use qt_gui::q_font_database::SystemFont;
use qt_gui::QGuiApplication;
use qt_network::q_abstract_socket::SocketState;

use super::Api;
use crate::binding::color::Colored;
use crate::binding::{RColor, RFont};
use crate::client::state::Mccp;
use crate::constants::branding;
use crate::script::{Callback, ScriptArg};

pub fn provide_api<'lua, M: mlua::UserDataMethods<'lua, Api>>(methods: &mut M) {
    methods.add_method("GetInfo", |lua, this, i| this.get_info(lua, i))
}

fn input_modifiers() -> i32 {
    let mut code = 0;
    let keyboard_mods = unsafe { QGuiApplication::keyboard_modifiers() };
    if keyboard_mods.test_flag(KeyboardModifier::ShiftModifier) {
        code |= 0x00001 | 0x00008;
    }
    if keyboard_mods.test_flag(KeyboardModifier::ControlModifier) {
        code |= 0x00002 | 0x00020;
    }
    if keyboard_mods.test_flag(KeyboardModifier::AltModifier) {
        code |= 0x00004 | 0x00080;
    }
    let mouse_mods = unsafe { QGuiApplication::mouse_buttons() };
    if mouse_mods.test_flag(MouseButton::LeftButton) {
        code |= 0x10000;
    }
    if mouse_mods.test_flag(MouseButton::RightButton) {
        code |= 0x20000;
    }
    if mouse_mods.test_flag(MouseButton::MiddleButton) {
        code |= 0x40000;
    }
    code
}

//#[api_provider]
impl Api {
    pub fn get_info<'lua>(&self, lua: &'lua Lua, i: c_int) -> Result<Value<'lua>> {
        let Api {
            output,
            paths,
            socket,
            state,
            world,
            ..
        } = &self;
        macro_rules! lua {
            ($e:expr) => {
                ScriptArg::to_arg($e, lua)
            };
        }
        match i {
            // strings - configuration
            1 => lua!(&world.site),
            2 => lua!(&world.name),
            3 => lua!(&world.player),
            4 => lua!(""), // send to world - file preamble
            5 => lua!(""), // send to world - file postamble
            6 => lua!(&world.send_line_preamble),
            7 => lua!(&world.send_line_postamble),
            8 => lua!(&world.notes),
            9 => lua!(&world.new_activity_sound),
            10 => lua!(&world.script_editor),
            11 => lua!(&world.log_file_preamble),
            12 => lua!(&world.log_file_postamble),
            13 => lua!(&world.log_preamble_input),
            14 => lua!(&world.log_preamble_notes),
            15 => lua!(&world.log_preamble_output),
            16 => lua!(&world.log_postamble_input),
            17 => lua!(&world.log_postamble_notes),
            18 => lua!(&world.log_postamble_output),
            19 => lua!(&world.speed_walk_filler),
            20 => lua!(&world.output_font),
            21 => lua!(&world.speed_walk_prefix),
            22 => lua!(&world.connect_text),
            23 => lua!(&world.input_font),
            24 => lua!(""), // paste to world - file preamble
            25 => lua!(""), // paste to world - file postamble
            26 => lua!(&world.send_line_preamble),
            27 => lua!(&world.send_line_postamble),
            28 => lua!("Lua"), // scripting language
            29 => lua!(Callback::Open),
            30 => lua!(Callback::Close),
            31 => lua!(Callback::Connect),
            32 => lua!(Callback::Disconnect),
            33 => lua!(Callback::GetFocus),
            34 => lua!(Callback::LoseFocus),
            35 => Ok(Value::Nil), // world script file name
            36 => lua!(&world.script_prefix),
            37 => lua!(&world.auto_say_string),
            38 => lua!(&world.auto_say_override_prefix),
            39 => lua!(""), // tab completion defaults
            40 => lua!(&world.auto_log_file_name),
            41 => lua!(""), // recall window - line preamble
            42 => lua!(&world.terminal_identification),
            43 => lua!(""), // mapping failure message
            44 => lua!(Callback::MxpStart),
            45 => lua!(Callback::MxpStop),
            46 => lua!(Callback::MxpError),
            47 => lua!(Callback::MxpOpenTag),
            48 => lua!(Callback::MxpCloseTag),
            49 => lua!(Callback::MxpSetVariable),
            50 => lua!(&world.beep_sound),

            // strings
            51 => lua!(&world.auto_log_file_name),
            52 => lua!(""), // last "immediate" script expression (TODO?)
            53 => lua!(""), // current status line mesage (TODO)
            54 => lua!(&world.world_script),
            55 => lua!(&world.name),
            56 => lua!(&paths.app),
            57 => lua!(&paths.worlds),
            58 => lua!(&paths.logs),
            59 => lua!(&paths.scripts),
            60 => lua!(&paths.plugins),
            61 => lua!(&paths.worlds),
            62 => lua!(socket.peer().ip().to_string()),
            63 => lua!(&gethostname::gethostname()),
            64 => lua!(""), // current directory
            65 => lua!(Callback::WorldSave),
            66 => lua!(&paths.base), // MUSHclient application directory
            67 => lua!(""),          // world file directory
            68 => lua!(&paths.base), // MUSHclient startup (initial) directory
            69 => lua!(""),          // translation file
            70 => lua!(""),          // locale
            71 => lua!(&RFont::global(StyleHint::Monospace)), // font used for fixed-pitch dialogs
            72 => lua!(branding::VERSION),
            73 => lua!(""), // compilation date/time (bad idea)
            74 => lua!(&paths.sounds),
            75 => lua!(""), // last telnet subnegotiation string received (TODO)
            76 => lua!(""), // special font pathname (TODO ?????)
            77 => lua!(OS), // theoretically Windows version debug string
            78 => lua!(""), // foreground image name
            79 => lua!(""), // background image name
            80 => lua!(""), // libpng version (eg. "1.2.31" TODO)
            81 => lua!(""), // libpng header (eg. " libpng version 1.2.31 - August 21, 2008") TODO
            82 => lua!(&paths.preferences),
            83 => lua!(str::from_utf8(&SQLITE_VERSION[..SQLITE_VERSION.len() - 1]).unwrap()),
            84 => lua!(""), // file browsing directory (TODO)
            85 => lua!(&paths.plugin_states),
            86 => lua!(""), // word under mouse on mouse menu click (TODO)
            87 => lua!(""), // last command sent to the MUD (TODO)
            88 => lua!(""), // window title (TODO)
            89 => lua!(""), // main window title

            // booleans
            101 => lua!(state.no_echo.get()),
            102 => lua!(false), // debug incoming packets
            103 => lua!(state.compressing.get()),
            104 => lua!(state.mxp_active.get()),
            105 => lua!(state.pueblo_active.get()),
            106 => lua!(socket.state() == SocketState::UnconnectedState),
            107 => lua!(socket.state() == SocketState::ConnectingState),
            108 => lua!(state.disconnect_ok.get()),
            109 => lua!(false), // trace flag (TODO?)
            110 => lua!(false), // script file changed (TODO?)
            111 => lua!(false), // world file is modified (TODO)
            112 => lua!(false), // automapper active (TODO)
            113 => lua!(true),  // world is active (TODO PRIORITY)
            114 => lua!(false), // output window paused (TODO)
            115 => lua!(false), // localization active (TODO)
            118 => lua!(false), // variables have changed (TODO?)
            119 => lua!(true),  // script engine is active (TODO?)
            120 => lua!(output.vertical_scroll_bar().is_displayed()),
            121 => lua!(true),  // high-resolution timer is available
            122 => lua!(true),  // sqlite3 library is thread-safe,
            123 => lua!(false), // are we processing "simulate" function (TODO?)
            124 => lua!(false), // is the current line being omitted (TODO)
            125 => lua!(false), // is the client in full-screen mode (TODO)

            // numbers
            201 => lua!(state.linecount.get()),
            202 => lua!(0), // lines received but not yet seen (new lines) (TODO)
            203 => lua!(0), // total lines sent (TODO)
            204 => lua!(0), // packets received (TODO)
            205 => lua!(0), // packet sent (TODO)
            206 => lua!(0), // total uncompressed bytes received (TODO)
            207 => lua!(0), // total compressed bytes received (TODO)
            208 => lua!(match state.mccp_ver.get() {
                None => 0,
                Some(Mccp::V1) => 1,
                Some(Mccp::V2) => 2,
            }),
            209 => lua!(0), // MXP error count (TODO)
            210 => lua!(0), // MXP tags received (TODO)
            211 => lua!(0), // MXP entities received (TODO)
            212 => lua!(world.output_font.metrics().height()),
            213 => lua!(world.output_font.metrics().average_char_width()),
            214 => lua!(world.input_font.metrics().height()),
            215 => lua!(world.input_font.metrics().average_char_width()),
            216 => lua!(0), // total bytes received (TODO)
            217 => lua!(0), // total bytes sent (TODO)
            218 => lua!(0), // count of variables (TODO)
            219 => lua!(0), // count of triggers (TODO)
            220 => lua!(0), // count of timers
            221 => lua!(0), // count of aliases (TODO)
            222 => lua!(0), // count of queued commands (TODO)
            223 => lua!(0), // count of mapper items (TODO)
            224 => lua!(0), // count of lines in output window (TODO)
            225 => lua!(0), // count of custom MXP elements
            226 => lua!(0), // count of custom MXP entities
            227 => lua!(match socket.state() {
                SocketState::HostLookupState => 1,
                SocketState::ConnectingState => 3,
                SocketState::ConnectedState => 8,
                _ => 0,
            }),
            228 => match socket.peer() {
                SocketAddr::V4(addr) => lua!(u32::from(*addr.ip())),
                SocketAddr::V6(addr) => match addr.ip().to_ipv4() {
                    Some(ip) => lua!(u32::from(ip)),
                    None => lua!(u128::from(*addr.ip())),
                },
            },
            229 => lua!(0), // proxy server TCP/IP address (after DNS lookup, as a number) (TODO)
            230 => lua!(0), // script execution depth
            231 => lua!(0), // log file size (0 if no log file open)
            232 => lua!(0.0), // high-performance counter output (in seconds) (TODO)
            233 => lua!(0.0), // time spent executing trigger matching (TODO)
            234 => lua!(0.0), // time spent executing alias matching (TODO)
            235 => lua!(1), // number of world windows open (TODO)
            236 => {
                let cursor = output.text_cursor();
                lua!(match cursor.selection() {
                    None => cursor.column() + 1,
                    Some(selection) => selection.start_cursor().column(),
                })
            }
            237 => {
                let cursor = output.text_cursor();
                lua!(match cursor.selection() {
                    None => 0,
                    Some(selection) => selection.end_cursor().column(),
                })
            }
            238 => lua!(5), // current "show state" for the first world window of this world (TODO)
            239 => lua!(0), // source of current scripted action (TODO)
            240 => lua!(output.font_metrics().average_char_width()),
            241 => lua!(output.font_metrics().height()),
            242 => lua!(0), // count of lines that had bad UTF-8 sequences in them (TODO)
            243 => lua!(RFont::from(SystemFont::FixedFont).size()),
            244 => lua!(0), // count of number of triggers that were evaluated (TODO)
            245 => lua!(0), // count of number of triggers that matched (TODO)
            246 => lua!(0), // count of number of aliases that were evaluated (TODO)
            247 => lua!(0), // count of number of aliases that matched (TODO)
            248 => lua!(0), // count of number of timers that fired (TODO)
            249 => lua!(output.height()), // main frame window height
            250 => lua!(output.width()), // main frame window width
            251 => lua!(0), // main toolbar window height (TODO)
            252 => lua!(0), // main toolbar window width (TODO)
            253 => lua!(0), // game toolbar window height (TODO)
            254 => lua!(0), // game toolbar window width (TODO)
            255 => lua!(0), // activity toolbar window height (TODO)
            256 => lua!(0), // activity toolbar window width (TODO)
            257 => lua!(0), // info bar window height (TODO)
            258 => lua!(0), // info bar window width (TODO)
            259 => lua!(0), // status bar window height (TODO)
            260 => lua!(0), // status bar window width (TODO)
            261 => lua!(output.height()), // world window non-client height
            262 => lua!(output.width()), // world window non-client width
            263 => lua!(output.height()), // world window client height
            264 => lua!(output.width()), // world window client width
            265 => lua!(0), // windows major version (TODO)
            266 => lua!(0), // window minor version (TODO)
            267 => lua!(0), // windows build number (TODO)
            268 => lua!(0), // windows platform ID (TODO)
            269 => lua!(0), // foreground image mode
            270 => lua!(0), // background image mode
            271 => lua!(&output.background_color()),

            272 => lua!(output.rect().left()), // text rectangle left
            273 => lua!(output.rect().top()),  // text rectangle top
            274 => lua!(output.rect().right()), // text rectangle right
            275 => lua!(output.rect().bottom()), // text rectangle bottom
            276 => lua!(0),                    // text rectangle border offset
            277 => lua!(0),                    // text rectangle border width
            278 => lua!(&output.background_color()), // text rectangle outside color
            279 => lua!(0),                    // text rectangle outside style
            280 => lua!(output.height()),      // output window client height
            281 => lua!(output.width()),       // output window client width
            282 => lua!(&RColor::from(GlobalColor::Transparent)), // text rectangle border color
            283 => lua!(output.cursor_position().x()),
            284 => lua!(output.cursor_position().y()),

            285 => lua!(true), // is output window available?
            286 => lua!(0),    // triggers matched this session (TODO)
            287 => lua!(0),    // aliases matched this session (TODO)
            288 => lua!(0),    // timers fired this session (TODO)
            289 => lua!(0),    // last line number terminated by IAC/GA or IAC/EOR

            290 => lua!(output.rect().left()), // actual text rectangle left
            291 => lua!(output.rect().top()),  // actual text rectangle top
            292 => lua!(output.rect().right()), // actual text rectangle right
            293 => lua!(output.rect().bottom()), // actual text rectangle bottom
            294 => lua!(input_modifiers()),
            295 => lua!(0), // number of times output window was redrawn (TODO)
            296 => lua!(output.vertical_scroll_bar().value()),
            297 => lua!(0), // resolution (frequency) of the high-resolution timer (TODO)
            298 => lua!(SQLITE_VERSION_NUMBER),
            299 => lua!(1200), // ansi code page number (Unicode)
            300 => lua!(437),  // oem code page number (MS-DOS United States)
            301 => lua!(&state.total_connect_duration.get()),
            302 => lua!(0), // time log file was last flushed to disk (TODO)
            303 => lua!(0), // when script file was last modified (TODO)
            304 => lua!(&SystemTime::now()),
            305 => lua!(0), // when client started executing (TODO)
            306 => lua!(0), // when this world was created/opened (TODO)

            310 => lua!(0), // newlines received from the MUD (lines terminated by a newline) (TODO)
            _ => Ok(Value::Nil),
        }
    }
}
