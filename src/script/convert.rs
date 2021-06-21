use cpp_core::{CppBox, CppDeletable, Ptr, Ref, StaticUpcast};
use mlua::{self, FromLuaMulti, LightUserData, Lua, MultiValue, ToLua, ToLuaMulti, Value};
use qt_core::{QBox, QObject, QPtr, QString};
use std::collections::BTreeMap;
use std::hash::{BuildHasher, Hash};
use std::ops::Deref;
use std::rc::Rc;
use std::str;

pub trait ScriptRes: for<'lua> FromLuaMulti<'lua> {}
impl<T: for<'lua> FromLuaMulti<'lua>> ScriptRes for T {}

pub trait ScriptArg {
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>>;
}

impl ScriptArg for &QString {
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        self.to_std_string().to_arg(lua)
    }
}

impl<T: ScriptArg> ScriptArg for Option<T> {
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        match self {
            Some(val) => val.to_arg(lua),
            None => Ok(Value::Nil),
        }
    }
}

macro_rules! impl_arg {
    ($t:ty) => {
        impl ScriptArg for $t {
            #[inline]
            fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
                ToLua::to_lua(self, lua)
            }
        }
    };
}

impl_arg!(bool);
impl_arg!(String);
impl_arg!(&'_ str);
impl_arg!(std::ffi::CString);
impl_arg!(&'_ std::ffi::CStr);
impl_arg!(i8);
impl_arg!(u8);
impl_arg!(i16);
impl_arg!(u16);
impl_arg!(i32);
impl_arg!(u32);
impl_arg!(i64);
impl_arg!(u64);
impl_arg!(i128);
impl_arg!(u128);
impl_arg!(isize);
impl_arg!(usize);
impl_arg!(f32);
impl_arg!(f64);
impl_arg!(LightUserData);

macro_rules! impl_arg_deref {
    ($t:ident) => (impl_arg_deref!{$t,});
    ($t:ident, $($bounds:path),*) => {
        impl<T> ScriptArg for $t<T>
        where
            for<'a> &'a T: ScriptArg,
            $(T: $bounds),*
        {
            fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
                self.deref().to_arg(lua)
            }
        }
        impl<T> ScriptArg for &$t<T>
        where
            for<'a> &'a T: ScriptArg,
            $(T: $bounds),*
        {
            fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
                self.deref().to_arg(lua)
            }
        }
    }
}

impl_arg_deref!(CppBox, CppDeletable);
impl_arg_deref!(Ptr);
impl_arg_deref!(QBox, CppDeletable, StaticUpcast<QObject>);
impl_arg_deref!(QPtr, StaticUpcast<QObject>);
impl_arg_deref!(Rc);
impl_arg_deref!(Ref);

fn create_table<'lua, K, V, I>(lua: &'lua Lua, cont: I) -> mlua::Result<Value<'lua>>
where
    K: ScriptArg,
    V: ScriptArg,
    I: IntoIterator<Item = (K, V)>,
{
    let args: mlua::Result<Vec<_>> = cont
        .into_iter()
        .map(|(k, v)| Ok((k.to_arg(lua)?, v.to_arg(lua)?)))
        .collect();
    lua.create_table_from(args?).map(Value::Table)
}

fn create_sequence<'lua, T, I>(lua: &'lua Lua, cont: I) -> mlua::Result<Value<'lua>>
where
    T: ScriptArg,
    I: IntoIterator<Item = T>,
{
    create_table(lua, cont.into_iter().enumerate().map(|(k, v)| (k + 1, v)))
}

impl<T: ScriptArg> ScriptArg for Vec<T> {
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        create_sequence(lua, self)
    }
}

impl<K: ScriptArg + Eq + Hash, V: ScriptArg, S: BuildHasher> ScriptArg
    for hashbrown::HashMap<K, V, S>
{
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        create_table(lua, self)
    }
}

impl<K: ScriptArg + Eq + Hash, V: ScriptArg, S: BuildHasher> ScriptArg
    for std::collections::HashMap<K, V, S>
{
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        create_table(lua, self)
    }
}

impl<K: ScriptArg + Ord, V: ScriptArg> ScriptArg for BTreeMap<K, V> {
    fn to_arg<'lua>(self, lua: &'lua Lua) -> mlua::Result<Value<'lua>> {
        create_table(lua, self)
    }
}

pub trait ScriptArgs {
    fn to_args<'lua>(self, lua: &'lua Lua) -> mlua::Result<MultiValue<'lua>>;
}

impl ScriptArgs for () {
    fn to_args<'lua>(self, _: &'lua Lua) -> mlua::Result<MultiValue<'lua>> {
        Ok(MultiValue::new())
    }
}

impl<T: ScriptArg> ScriptArgs for T {
    fn to_args<'lua>(self, lua: &'lua Lua) -> mlua::Result<MultiValue<'lua>> {
        ToLuaMulti::to_lua_multi(self.to_arg(lua), lua)
    }
}

macro_rules! peel_args {
    ($name:ident, $($other:ident,)*) => (impl_args! { $($other,)* })
}

macro_rules! impl_args {
    () => ();
    ($($name:ident,)+) => {
        impl<$($name,)*> ScriptArgs for ($($name,)*)
        where $($name: ScriptArg,)*
        {
            #[allow(non_snake_case)]
            fn to_args<'lua>(self, lua: &'lua Lua) -> mlua::Result<MultiValue<'lua>> {
                let ($($name,)*) = self;

                ToLuaMulti::to_lua_multi((
                    $(ScriptArg::to_arg($name, lua)?,)*
                ), lua)
            }
        }
        peel_args! { $($name,)+ }
    }
}

impl_args! {A, B, C, D, E, F, G, H, I, J, K, L, M, N, O,}
