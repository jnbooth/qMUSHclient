use std::os::raw::c_int;

use mlua::{Lua, Result, Value};

use crate::api::Api;
use crate::script::ScriptArg;

#[api_provider]
impl Api {
    #[api("WindowInfo")]
    pub fn window_info<'lua>(
        &self,
        lua: &'lua Lua,
        window_name: String,
        i: c_int,
    ) -> Result<Value<'lua>> {
        match i {
            0 => i.to_arg(lua),
            _ => window_name.to_arg(lua),
        }
    }
}
