use std::os::raw::c_int;

use mlua::{self, lua_CFunction, lua_State, Lua, LuaOptions};

extern "C" {
    fn luaopen_bc(L: *mut lua_State) -> c_int;
    fn luaopen_lpeg(L: *mut lua_State) -> c_int;
    fn luaopen_rex_pcre2(L: *mut lua_State) -> c_int;
    fn luaopen_lsqlite3(L: *mut lua_State) -> c_int;
}

fn load_libs(lua: &Lua) -> mlua::Result<()> {
    unsafe {
        load_lib(lua, "bc", luaopen_bc)?;
        load_lib(lua, "lpeg", luaopen_lpeg)?;
        load_lib(lua, "rex", luaopen_rex_pcre2)?;
        load_lib(lua, "lsqlite3", luaopen_lsqlite3)?;
        load_lib(lua, "sqlite3", luaopen_lsqlite3)?; // for backward compatibility
    }
    Ok(())
}

unsafe fn load_lib(lua: &Lua, name: &str, f: lua_CFunction) -> mlua::Result<()> {
    let opener = unsafe { lua.create_c_function(f) }?;
    let _: mlua::Value = lua.load_from_function(name, opener)?;
    Ok(())
}

pub fn new_lua() -> mlua::Result<Lua> {
    unsafe {
        let lua = Lua::unsafe_new_with(mlua::StdLib::ALL_SAFE, LuaOptions::default());
        load_libs(&lua)?;
        Ok(lua)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_lib(s: &[u8]) {
        let lua = new_lua().expect("Error loading libraries");
        if let Err(e) = lua.load(s).exec() {
            panic!("{}", e.to_string());
        }
    }

    #[test]
    fn test_lbc() {
        test_lib(include_bytes!("../../c_lib/lbc/test.lua"));
    }

    #[test]
    fn test_lpeg() {
        test_lib(include_bytes!("../../c_lib/lpeg/test.lua"));
    }

    #[test]
    fn test_lsqlite3() {
        test_lib(include_bytes!("../../c_lib/lsqlite3/test.lua"));
    }
}
