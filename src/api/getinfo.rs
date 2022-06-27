use std::env::consts::OS;
use std::os::raw::c_int;
use std::str;

use libsqlite3_sys::SQLITE_VERSION;
use mlua::{Lua, Result, Value};
use qt_gui::q_font::StyleHint;
use qt_network::q_abstract_socket::SocketState;

use super::Api;
use crate::binding::RFont;
use crate::constants::branding;
use crate::script::{Callback, ScriptArg};

pub fn provide_api<'lua, M: mlua::UserDataMethods<'lua, Api>>(methods: &mut M) {
    methods.add_method("GetInfo", |lua, this, i| this.get_info(lua, i))
}

//#[api_provider]
impl Api {
    pub fn get_info<'lua>(&self, lua: &'lua Lua, i: c_int) -> Result<Value<'lua>> {
        let state = &*self.state;
        let world = &*self.world;
        macro_rules! lua {
            ($e:expr) => {
                ScriptArg::to_arg($e, lua)
            };
        }
        macro_rules! lshow {
            ($e:expr) => {
                ScriptArg::to_arg($e.to_string(), lua)
            };
        }
        macro_rules! world {
            ($field:ident) => {
                ScriptArg::to_arg(world.$field, lua)
            };
            (&$field:ident) => {
                ScriptArg::to_arg(&world.$field, lua)
            };
        }
        macro_rules! flag {
            ($field:ident) => {
                ScriptArg::to_arg(state.$field.get(), lua)
            };
        }
        match i {
            // strings - configuration
            1 => world!(&site),
            2 => world!(&name),
            3 => world!(&player),
            4 => lua!(""), // send to world - file preamble
            5 => lua!(""), // send to world - file postamble
            6 => world!(&send_line_preamble),
            7 => world!(&send_line_postamble),
            8 => world!(&notes),
            9 => world!(&new_activity_sound),
            10 => world!(&script_editor),
            11 => world!(&log_file_preamble),
            12 => world!(&log_file_postamble),
            13 => world!(&log_preamble_input),
            14 => world!(&log_preamble_notes),
            15 => world!(&log_preamble_output),
            16 => world!(&log_postamble_input),
            17 => world!(&log_postamble_notes),
            18 => world!(&log_postamble_output),
            19 => world!(&speed_walk_filler),
            20 => world!(&output_font),
            21 => world!(&speed_walk_prefix),
            22 => world!(&connect_text),
            23 => world!(&input_font),
            24 => lua!(""), // paste to world - file preamble
            25 => lua!(""), // paste to world - file postamble
            26 => world!(&send_line_preamble),
            27 => world!(&send_line_postamble),
            28 => lua!("Lua"), // scripting language
            29 => lua!(Callback::Open),
            30 => lua!(Callback::Close),
            31 => lua!(Callback::Connect),
            32 => lua!(Callback::Disconnect),
            33 => lua!(Callback::GetFocus),
            34 => lua!(Callback::LoseFocus),
            35 => Ok(Value::Nil), // world script file name
            36 => world!(&script_prefix),
            37 => world!(&auto_say_string),
            38 => world!(&auto_say_override_prefix),
            39 => lua!(""), // tab completion defaults
            40 => world!(&auto_log_file_name),
            41 => lua!(""), // recall window - line preamble
            42 => world!(&terminal_identification),
            43 => lua!(""), // mapping failure message
            44 => lua!(Callback::MxpStart),
            45 => lua!(Callback::MxpStop),
            46 => lua!(Callback::MxpError),
            47 => lua!(Callback::MxpOpenTag),
            48 => lua!(Callback::MxpCloseTag),
            49 => lua!(Callback::MxpSetVariable),
            50 => world!(&beep_sound),

            // strings
            51 => world!(&auto_log_file_name),
            52 => lua!(""), // last "immediate" script expression (TODO?)
            53 => lua!(""), // current status line mesage (TODO)
            54 => world!(&world_script),
            55 => world!(&name),
            56 => lua!(&self.paths.app),
            57 => lua!(&self.paths.worlds),
            58 => lua!(&self.paths.logs),
            59 => lua!(&self.paths.scripts),
            60 => lua!(&self.paths.plugins),
            61 => lua!(&self.paths.worlds),
            62 => lshow!(self.socket.peer().ip()),
            63 => lua!(&gethostname::gethostname()),
            64 => lua!(""), // current directory
            65 => lua!(Callback::WorldSave),
            66 => lua!(&self.paths.base), // MUSHclient application directory
            67 => lua!(""),               // world file directory
            68 => lua!(&self.paths.base), // MUSHclient startup (initial) directory
            69 => lua!(""),               // translation file
            70 => lua!(""),               // locale
            71 => lua!(&RFont::global(StyleHint::Monospace)), // font used for fixed-pitch dialogs
            72 => lua!(branding::VERSION),
            73 => lua!(""), // compilation date/time (bad idea)
            74 => lua!(&self.paths.sounds),
            75 => lua!(""), // last telnet subnegotiation string received (TODO)
            76 => lua!(""), // special font pathname (TODO ?????)
            77 => lua!(OS), // theoretically Windows version debug string
            78 => lua!(""), // foreground image name
            79 => lua!(""), // background image name
            80 => lua!(""), // libpng version (eg. "1.2.31" TODO)
            81 => lua!(""), // libpng header (eg. " libpng version 1.2.31 - August 21, 2008") TODO
            82 => lua!(&self.paths.preferences),
            83 => lua!(str::from_utf8(&SQLITE_VERSION[..SQLITE_VERSION.len() - 1]).unwrap()),
            84 => lua!(""), // file browsing directory (TODO)
            85 => lua!(&self.paths.plugin_states),
            86 => lua!(""), // word under mouse on mouse menu click (TODO)
            87 => lua!(""), // last command sent to the MUD (TODO)
            88 => lua!(""), // window title (TODO)
            89 => lua!(""), // main window title

            // booleans
            101 => flag!(no_echo),
            102 => lua!(false), // debug incoming packets
            103 => flag!(compressing),
            104 => flag!(mxp_active),
            105 => flag!(pueblo_active),
            106 => lua!(self.socket.state() == SocketState::UnconnectedState),
            107 => lua!(self.socket.state() == SocketState::ConnectingState),
            108 => lua!(self.socket.state() == SocketState::ConnectedState), // ok to disconnect
            109 => lua!(false),                                              // trace flag (TODO?)
            110 => lua!(false), // script file changed (TODO?)
            111 => lua!(false), // world file is modified (TODO)
            112 => lua!(false), // automapper active (TODO)
            113 => lua!(true),  // world is active (TODO PRIORITY)
            114 => lua!(false), // output window paused (TODO)
            115 => lua!(false), // localization active (TODO)
            118 => lua!(false), // variables have changed (TODO?)
            119 => lua!(true),  // script engine is active (TODO?)
            120 => lua!(self.output.vertical_scroll_bar().is_displayed()),
            121 => lua!(true),  // high-resolution timer is available
            122 => lua!(true),  // sqlite3 library is thread-safe,
            123 => lua!(false), // are we processing "simulate" function (TODO?)
            124 => lua!(false), // is the current line being omitted (TODO)
            125 => lua!(false), // is the client in full-screen mode (TODO)

            // numbers
            212 => lua!(self.world.output_font.size()),

            239 => lua!(0), // source of current scripted action
            240 => lua!(self.output.font_metrics().average_char_width()),

            272 => lua!(self.input.rect().left()),
            273 => lua!(self.input.rect().top()),
            274 => lua!(self.input.rect().right()),
            275 => lua!(self.input.rect().bottom()),
            280 => lua!(self.output.width()),
            281 => lua!(self.output.height()),
            _ => Ok(Value::Nil),
        }
    }
}
