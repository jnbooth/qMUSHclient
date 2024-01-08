use std::hash::Hash;

use mlua::{self, FromLuaMulti, IntoLuaMulti, Lua, UserData, UserDataMethods};

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MethodGatherer(Vec<String>);

impl MethodGatherer {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn into_vec(self) -> Vec<String> {
        self.0
    }
}

impl<'lua, T: UserData> UserDataMethods<'lua, T> for MethodGatherer {
    fn add_method<M, A, R>(&mut self, name: impl AsRef<str>, _: M)
    where
        M: Fn(&'lua Lua, &T, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
        self.0.push(name.as_ref().to_owned());
    }

    fn add_method_mut<M, A, R>(&mut self, name: impl AsRef<str>, _: M)
    where
        M: FnMut(&'lua Lua, &mut T, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
        self.0.push(name.as_ref().to_owned());
    }

    fn add_function<F, A, R>(&mut self, _: impl AsRef<str>, _: F)
    where
        F: Fn(&'lua Lua, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }

    fn add_function_mut<F, A, R>(&mut self, _: impl AsRef<str>, _: F)
    where
        F: FnMut(&'lua Lua, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }

    fn add_meta_method<M, A, R>(&mut self, _: impl AsRef<str>, _: M)
    where
        M: Fn(&'lua Lua, &T, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }

    fn add_meta_method_mut<M, A, R>(&mut self, _: impl AsRef<str>, _: M)
    where
        M: FnMut(&'lua Lua, &mut T, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }

    fn add_meta_function<F, A, R>(&mut self, _: impl AsRef<str>, _: F)
    where
        F: Fn(&'lua Lua, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }

    fn add_meta_function_mut<F, A, R>(&mut self, _: impl AsRef<str>, _: F)
    where
        F: FnMut(&'lua Lua, A) -> mlua::Result<R> + 'static,
        A: FromLuaMulti<'lua>,
        R: IntoLuaMulti<'lua>,
    {
    }
}
