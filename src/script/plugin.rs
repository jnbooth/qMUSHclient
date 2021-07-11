use std::fmt::Write;
use std::hash::Hash;
use std::path::PathBuf;
use std::str::{self, FromStr};

use chrono::{Date, Local};
use mlua::{
    self, AnyUserData, FromLuaMulti, Function, Lua, MetaMethod, ToLuaMulti, UserData,
    UserDataMethods, Value,
};
use qt_core::QString;
use qt_widgets::q_message_box::Icon;
use qt_widgets::QMessageBox;
use uuid::Uuid;

use super::callback::Callback;
use super::convert::{ScriptArgs, ScriptRes};
use crate::enums::{Enum, EnumSet};
use crate::tr::TrContext;
use crate::Version;

pub type PluginId = Uuid;

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
    fn add_method<S, A, R, M>(&mut self, name: &S, _: M)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + Fn(&'lua Lua, &T, A) -> mlua::Result<R>,
    {
        self.0.push(
            str::from_utf8(name.as_ref())
                .expect("UserData method name is invalid UTF-8")
                .to_owned(),
        );
    }

    fn add_method_mut<S, A, R, M>(&mut self, name: &S, _: M)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + FnMut(&'lua Lua, &mut T, A) -> mlua::Result<R>,
    {
        self.0.push(
            str::from_utf8(name.as_ref())
                .expect("UserData method name is invalid UTF-8")
                .to_owned(),
        );
    }

    fn add_function<S, A, R, F>(&mut self, _: &S, _: F)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + Fn(&'lua Lua, A) -> mlua::Result<R>,
    {
    }

    fn add_function_mut<S, A, R, F>(&mut self, _: &S, _: F)
    where
        S: ?Sized + AsRef<[u8]>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + FnMut(&'lua Lua, A) -> mlua::Result<R>,
    {
    }

    fn add_meta_method<S, A, R, M>(&mut self, _: S, _: M)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + Fn(&'lua Lua, &T, A) -> mlua::Result<R>,
    {
    }

    fn add_meta_method_mut<S, A, R, M>(&mut self, _: S, _: M)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        M: 'static + FnMut(&'lua Lua, &mut T, A) -> mlua::Result<R>,
    {
    }

    fn add_meta_function<S, A, R, F>(&mut self, _: S, _: F)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + Fn(&'lua Lua, A) -> mlua::Result<R>,
    {
    }

    fn add_meta_function_mut<S, A, R, F>(&mut self, _: S, _: F)
    where
        S: Into<MetaMethod>,
        A: FromLuaMulti<'lua>,
        R: ToLuaMulti<'lua>,
        F: 'static + FnMut(&'lua Lua, A) -> mlua::Result<R>,
    {
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// World plugins
pub struct PluginMetadata {
    /// Plugin name.
    pub name: String,
    /// Who wrote it?
    pub author: String,
    /// Short description of the plugin's functionality.
    pub purpose: String,
    /// Long description of the plugin's functionality.
    pub description: String,
    /// Script source (i.e. from <script> tags).
    pub script: String,
    /// File that contains the plugin.
    pub source: PathBuf,
    /// Unique ID.
    pub id: PluginId,
    /// Date written.
    pub written: Date<Local>,
    /// Date last modified.
    pub modified: Date<Local>,
    /// Plugin version.
    pub version: Version,
    /// Minimum qMUSHclient version required.
    pub client_required: Version,
    /// Date installed.
    pub installed: Date<Local>,
    /// Evaluation order. Lower is sooner.
    /// Negative sequences are evaluated before the main world triggers/aliases.
    pub sequence: i16,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseErrorTODO;

impl FromStr for PluginMetadata {
    type Err = ParseErrorTODO;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: "TODO".to_string(),
            author: "TODO".to_string(),
            purpose: "TODO".to_string(),
            description: "TODO".to_string(),
            script: s.to_string(),
            source: PathBuf::new(),
            id: PluginId::new_v4(),
            written: Local::today(),
            modified: Local::today(),
            version: Version(0),
            client_required: Version(0),
            installed: Local::today(),
            sequence: 0,
        })
    }
}

#[derive(TrContext)]
pub struct Plugin {
    metadata: PluginMetadata,
    /// True if active.
    enabled: bool,
    /// Which callbacks it responds to.
    callbacks: EnumSet<Callback>,
    /// Script engine for script.
    engine: Lua,
}

impl Plugin {
    pub fn load(engine: Lua, metadata: PluginMetadata) -> Result<Self, mlua::Error> {
        if let Err(e) = engine.load(&metadata.script).exec() {
            Self::alert_error(&metadata, &e);
            return Err(e);
        }
        let globals = engine.globals();

        let callbacks: EnumSet<Callback> = Callback::enumerate()
            .filter(|cb| matches!(globals.raw_get(cb.to_str()), Ok(Value::Function(..))))
            .collect();

        std::mem::drop(globals);

        Ok(Plugin {
            metadata,
            enabled: true,
            callbacks,
            engine,
        })
    }

    fn alert_error(metadata: &PluginMetadata, error: &mlua::Error) {
        unsafe {
            // TODO make this more complicated
            let msgbox = QMessageBox::new();
            msgbox.set_icon(Icon::Warning);
            msgbox.set_text(&tr!("Script error in {}", metadata.name));
            msgbox.set_informative_text(&QString::from_std_str(&error.to_string()));
            msgbox.exec();
        }
    }

    fn try_call<'lua, A, R>(engine: &'lua Lua, cb: Callback, args: A) -> mlua::Result<R>
    where
        A: ScriptArgs,
        R: FromLuaMulti<'lua>,
    {
        let f: Function<'lua> = engine.globals().raw_get(cb.to_str())?;
        f.call(args.to_args(&engine)?)
    }

    pub fn call<'lua, A, R>(&'lua mut self, cb: Callback, args: A) -> mlua::Result<R>
    where
        A: ScriptArgs,
        R: FromLuaMulti<'lua>,
    {
        let res = Self::try_call(&self.engine, cb, args);
        if let Err(e) = &res {
            Self::alert_error(&self.metadata, &e);
            self.enabled = false;
        }
        res
    }
}

pub trait CloneWith<T> {
    fn clone_with(&self, with: T) -> Self;
}

impl<T: Clone, C> CloneWith<C> for T {
    fn clone_with(&self, _: C) -> Self {
        self.clone()
    }
}

pub struct PluginHandler<U: 'static + UserData + for<'a> CloneWith<&'a PluginMetadata>> {
    userdata: U,
    initialize: String,
    plugins: Vec<Plugin>,
}

const USERDATA_KEY: &str = "__ud";

const fn truthy(value: &Value) -> bool {
    match *value {
        Value::Nil | Value::Boolean(false) | Value::Integer(0) => false,
        Value::Number(f) => f as u64 == 0,
        _ => true,
    }
}

impl<U: 'static + UserData + for<'a> CloneWith<&'a PluginMetadata>> PluginHandler<U> {
    pub fn new(userdata: U) -> Self {
        let mut gatherer = MethodGatherer::new();
        U::add_methods(&mut gatherer);

        let mut initialize = String::new();
        for me in gatherer.into_vec() {
            writeln!(
                initialize,
                "_G[{:?}] = function(...) __{}[{:?}](__{},...) end",
                me, USERDATA_KEY, me, USERDATA_KEY,
            )
            .ok();
        }

        Self {
            userdata,
            initialize,
            plugins: Vec::new(),
        }
    }

    fn iter_mut(&mut self, cb: Callback) -> impl Iterator<Item = &mut Plugin> {
        self.plugins
            .iter_mut()
            .filter(move |p| p.enabled && p.callbacks.contains(cb))
    }

    pub fn load_plugin(&mut self, metadata: PluginMetadata) -> mlua::Result<()> {
        if let Some(old) = self
            .plugins
            .iter()
            .position(|x| x.metadata.source == metadata.source)
        {
            self.plugins.remove(old);
        }
        let engine = crate::ffi::lua::new_lua()?;

        engine
            .globals()
            .set(USERDATA_KEY, self.userdata.clone_with(&metadata))?;
        engine.load(&self.initialize).exec()?;
        self.plugins.push(Plugin::load(engine, metadata)?);
        Ok(())
    }

    pub fn remove(&mut self, id: &PluginId) {
        if let Some(old) = self.plugins.iter().position(|x| &x.metadata.id == id) {
            self.plugins.remove(old);
        }
    }

    pub fn alter_userdata<F: FnMut(&mut U)>(&mut self, mut f: F) {
        f(&mut self.userdata);
        for plugin in &self.plugins {
            // Plugins have no reason to reassign their userdata global variable, but if they do,
            // they are left alone. The worst that could happen is they rename or hide it and
            // end up using an Api instance with outdated World information, which is not really
            // a problem, and more importantly, not our problem!
            if let Ok(udref) = plugin
                .engine
                .globals()
                .raw_get::<_, AnyUserData>(USERDATA_KEY)
            {
                if let Ok(mut ud) = udref.borrow_mut::<U>() {
                    f(&mut ud);
                }
            }
        }
    }

    pub fn _send_to_first<A: ScriptArgs + Clone>(&mut self, cb: Callback, args: A) -> bool {
        self.iter_mut(cb)
            .any(|p| p.call(cb, args.clone()).unwrap_or(false))
    }

    pub fn receive_from_all<A>(&mut self, cb: Callback, mut args: A) -> A
    where
        A: 'static + ScriptArgs + ScriptRes + Clone,
    {
        for plugin in self.iter_mut(cb) {
            if let Ok(new_args) = plugin.call(cb, args.clone()) {
                args = new_args;
            }
        }
        args
    }

    pub fn send_to_all<A: ScriptArgs + Clone>(&mut self, cb: Callback, args: A) {
        for plugin in self.iter_mut(cb) {
            plugin.call(cb, args.clone()).unwrap_or(());
        }
    }

    pub fn send_to_all_until<A: ScriptArgs + Clone>(
        &mut self,
        cb: Callback,
        args: A,
        stop: EnumSet<bool>,
    ) -> bool {
        for plugin in self.iter_mut(cb) {
            if let Ok(res) = plugin.call(cb, args.clone()) {
                let truth = truthy(&res);
                if stop.contains(truth) {
                    return truth;
                }
            }
        }
        !stop.contains(true)
    }
}
