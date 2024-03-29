use std::os::raw::c_int;

use mlua::{Lua, Result, Value};
use qmushclient_scripting::ScriptArg;

use crate::api::Api;

#[api_provider]
impl Api {
    /// Implements https://www.gammon.com.au/scripts/doc.php?function=WindowInfo
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
