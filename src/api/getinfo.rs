use std::os::raw::c_int;

use mlua::{Lua, Result, Value};
use qt_gui::q_font::StyleHint;

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
            35 => lua!(""), // world script file name
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

            // strings - calculated at runtime
            51 => world!(&auto_log_file_name),
            52 => lua!(""), // last "immediate" script expression (TODO?)
            53 => lua!(""), // current status line mesage (TODO)
            54 => world!(&world_script),
            55 => world!(&name),
            56 => lua!(""), // MUSHclient application path name (TODO?)
            57 => lua!(""), // world files default path (directory) (TODO)
            58 => lua!(""), // log files default path (directory) (TODO)
            59 => lua!(""), // script files default path (directory) (TODO)
            60 => lua!(""), // plugin files default path (directory) (TODO)
            61 => lua!(""), // world files default path (directory) (TODO)
            62 => lshow!(self.socket.peer().ip()),
            63 => lua!(&gethostname::gethostname()),
            64 => lua!(""), // current directory
            65 => lua!(Callback::WorldSave),
            66 => lua!(""), // MUSHclient application directory
            67 => lua!(""), // world file directory
            68 => lua!(""), // MUSHclient startup (initial) directory
            69 => lua!(""), // translation file
            70 => lua!(""), // locale
            71 => lua!(&RFont::global(StyleHint::Monospace)), // font used for fixed-pitch dialogs
            72 => lua!(branding::VERSION),
            _ => Ok(Value::Nil),
        }
    }
}
