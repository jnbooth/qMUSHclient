use mlua::{Lua, Result, ToLua, Value};

use crate::api::Api;

#[api_provider]
impl Api {
    #[api("GetVariable")]
    pub fn get_variable<'lua>(&self, lua: &'lua Lua, key: String) -> Result<Value<'lua>> {
        match self.variables.borrow().get(&key) {
            Some(val) => val.as_str().to_lua(lua),
            None => Ok(Value::Nil),
        }
    }

    #[api("SetVariable")]
    pub fn set_variable(&self, key: String, val: String) {
        self.variables.borrow_mut().insert(key, val);
    }
}
