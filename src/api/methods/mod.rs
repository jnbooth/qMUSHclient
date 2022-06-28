use crate::api::Api;

mod get_info;
mod print;
mod triggers;
mod variables;
mod windows;

pub fn provide_api<'lua, M: mlua::UserDataMethods<'lua, Api>>(methods: &mut M) {
    get_info::provide_api(methods);
    print::provide_api(methods);
    triggers::provide_api(methods);
    variables::provide_api(methods);
    windows::provide_api(methods);
}
