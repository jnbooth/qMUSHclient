use std::os::raw::c_int;

pub use libsqlite3_sys::{SQLITE_VERSION, SQLITE_VERSION_NUMBER};
use mlua::lua_State;

extern crate libsqlite3_sys;
extern crate luajit_src;
extern crate pcre2_sys;

extern "C-unwind" {
    pub fn luaopen_bc(L: *mut lua_State) -> c_int;
    pub fn luaopen_lpeg(L: *mut lua_State) -> c_int;
    pub fn luaopen_rex_pcre2(L: *mut lua_State) -> c_int;
    pub fn luaopen_lsqlite3(L: *mut lua_State) -> c_int;
}
