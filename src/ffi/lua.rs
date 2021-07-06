use std::os::raw::c_int;

use mlua::{self, lua_CFunction, lua_State, Lua};

extern "C" {
    fn luaopen_bc(L: *mut lua_State) -> c_int;
    fn luaopen_lpeg(L: *mut lua_State) -> c_int;
    fn luaopen_rex_pcre2(L: *mut lua_State) -> c_int;
    fn luaopen_lsqlite3(L: *mut lua_State) -> c_int;
}

pub fn load_libs(lua: &Lua) -> mlua::Result<()> {
    unsafe {
        load_lib(lua, "bc", luaopen_bc)?;
        load_lib(lua, "lpeg", luaopen_lpeg)?;
        load_lib(lua, "rex", luaopen_rex_pcre2)?;
        load_lib(lua, "sqlite3", luaopen_lsqlite3)?;
    }
    Ok(())
}

unsafe fn load_lib(lua: &Lua, name: &str, f: lua_CFunction) -> mlua::Result<()> {
    let opener = unsafe { lua.create_c_function(f) }?;
    let _: mlua::Value = lua.load_from_function(name, opener)?;
    Ok(())
}
