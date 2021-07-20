use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter, Write};
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;
use std::{io, mem, str};

use mlua::{
    self, AnyUserData, FromLuaMulti, Function, Lua, MetaMethod, ToLuaMulti, UserData,
    UserDataMethods, Value,
};
use qt_core::QString;
use qt_widgets::q_message_box::Icon;
use qt_widgets::QMessageBox;

use super::callback::Callback;
use super::convert::{ScriptArgs, ScriptRes};
use super::{PluginMetadata, PluginPack};
use crate::binding::RColor;
use crate::enums::{Enum, EnumSet};
use crate::script::{Alias, SendRequest, Timer, Trigger};
use crate::tr::TrContext;
use crate::world::World;

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

#[derive(TrContext)]
pub struct Plugin {
    metadata: PluginMetadata,
    /// True if active.
    enabled: bool,
    /// Which callbacks it responds to.
    callbacks: EnumSet<Callback>,
    /// Script engine for script.
    engine: Lua,
    triggers: Vec<Trigger>,
    aliases: Vec<Alias>,
    timers: Vec<Timer>,
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.eq(&other.metadata)
    }
}

impl Eq for Plugin {}

impl PartialOrd for Plugin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.metadata.sequence.partial_cmp(&other.metadata.sequence)
    }
}

impl Ord for Plugin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata.sequence.cmp(&other.metadata.sequence)
    }
}

impl Plugin {
    pub fn load(engine: Lua, pack: PluginPack) -> mlua::Result<Self> {
        if let Err(e) = engine.load(&pack.script).exec() {
            Self::alert_error(&pack.metadata, &e);
            return Err(e);
        }
        let globals = engine.globals();

        let callbacks = Callback::enumerate(..)
            .filter(|cb| matches!(globals.raw_get(cb.to_str()), Ok(Value::Function(..))))
            .collect();

        mem::drop(globals);

        Ok(Plugin {
            metadata: pack.metadata,
            enabled: true,
            callbacks,
            engine,
            triggers: pack.triggers,
            aliases: pack.aliases,
            timers: pack.timers,
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

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Indexed<T> {
    index: PluginIndex,
    val: T,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Indexer<T>(Vec<Indexed<T>>);

impl<T> Indexer<T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn replace<'a, I>(&mut self, index: PluginIndex, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + Clone,
    {
        self.0.retain(|x| x.index != index);
        self.extend(index, iter);
    }

    pub fn extend<'a, I>(&mut self, index: PluginIndex, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
        T: 'a + Clone,
    {
        let new_iter = iter.into_iter().map(|content| Indexed {
            index,
            val: content.to_owned(),
        });
        self.0.extend(new_iter);
    }

    pub fn sort(&mut self)
    where
        T: Ord,
    {
        self.0.sort_unstable();
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (PluginIndex, &mut T)> {
        self.0
            .iter_mut()
            .map(|indexed| (indexed.index, &mut indexed.val))
    }
}

pub struct PluginHandler<U: 'static + UserData + for<'a> CloneWith<&'a PluginMetadata>> {
    userdata: U,
    initialize: String,
    plugins: Vec<Plugin>,
    triggers: Indexer<Trigger>,
    aliases: Indexer<Alias>,
    timers: Indexer<Timer>,
}

const USERDATA_KEY: &str = "__ud";

const fn truthy(value: &Value) -> bool {
    match *value {
        Value::Nil | Value::Boolean(false) | Value::Integer(0) => false,
        Value::Number(f) => f as u64 == 0,
        _ => true,
    }
}

#[derive(Debug)]
pub enum LoadError {
    File(io::Error),
    Xml(quick_xml::DeError),
    Script(mlua::Error),
}
impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        Self::File(value)
    }
}
impl From<quick_xml::DeError> for LoadError {
    fn from(value: quick_xml::DeError) -> Self {
        Self::Xml(value)
    }
}
impl From<mlua::Error> for LoadError {
    fn from(value: mlua::Error) -> Self {
        Self::Script(value)
    }
}
impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(e) => write!(f, "{}", e),
            Self::Xml(e) => write!(f, "{}", e),
            Self::Script(e) => write!(f, "{}", e),
        }
    }
}
impl StdError for LoadError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::File(e) => Some(e),
            Self::Xml(e) => Some(e),
            Self::Script(e) => Some(e),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TriggerRequests<'a> {
    pub send: Vec<SendRequest<'a>>,
    pub hide: bool,
    pub foreground: Option<&'a RColor>,
    pub background: Option<&'a RColor>,
    pub sounds: Vec<&'a Path>,
    pub make_bold: bool,
    pub make_italic: bool,
    pub make_underline: bool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PluginIndex(usize);

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
            triggers: Indexer::new(),
            aliases: Indexer::new(),
            timers: Indexer::new(),
        }
    }

    pub fn clear(&mut self) {
        self.plugins.clear();
    }

    fn iter_mut(&mut self, cb: Callback) -> impl Iterator<Item = &mut Plugin> {
        self.plugins
            .iter_mut()
            .filter(move |p| p.enabled && p.callbacks.contains(cb))
    }

    pub fn load_plugin_file(&mut self, path: &Path) -> Result<(), LoadError> {
        let file = File::open(path)?;
        let pack: PluginPack = quick_xml::de::from_reader(BufReader::new(file))?;
        self.load_plugin(pack)?;
        Ok(())
    }

    pub fn load_plugin(&mut self, pack: PluginPack) -> mlua::Result<()> {
        self.plugins.push(self.init_plugin(pack)?);
        Ok(())
    }

    fn init_plugin(&self, pack: PluginPack) -> mlua::Result<Plugin> {
        let engine = crate::ffi::lua::new_lua()?;

        engine
            .globals()
            .set(USERDATA_KEY, self.userdata.clone_with(&pack.metadata))?;
        engine.load(&self.initialize).exec()?;
        Plugin::load(engine, pack)
    }

    pub fn sort(&mut self) {
        self.triggers.clear();
        self.aliases.clear();
        self.timers.clear();

        self.plugins.sort_unstable();

        for (i, plugin) in self.plugins.iter().enumerate() {
            self.triggers.extend(PluginIndex(i), &plugin.triggers);
            self.triggers.sort();
            self.aliases.extend(PluginIndex(i), &plugin.aliases);
            self.aliases.sort();
            self.timers.extend(PluginIndex(i), &plugin.timers);
            self.timers.sort();
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

    pub fn update_world_plugin(&mut self, old: &Rc<World>, new: &Rc<World>) -> mlua::Result<()> {
        let i = match self.plugins.iter().position(|x| x.metadata.is_world_plugin) {
            Some(i) => PluginIndex(i),
            None => return self.load_plugin(new.world_plugin()),
        };
        if old.world_script != new.world_script {
            let old_plugin = &mut self.plugins[i.0];
            let pack = PluginPack {
                metadata: mem::take(&mut old_plugin.metadata),
                triggers: mem::take(&mut old_plugin.triggers),
                aliases: mem::take(&mut old_plugin.aliases),
                timers: mem::take(&mut old_plugin.timers),
                script: new.world_script.clone(),
            };
            self.plugins[i.0] = self.init_plugin(pack)?;
        }
        if old.triggers != new.triggers {
            self.triggers.replace(i, &new.triggers);
            self.triggers.sort();
        }
        if old.aliases != new.aliases {
            self.aliases.replace(i, &new.aliases);
            self.aliases.sort();
        }
        if old.timers != new.timers {
            self.timers.replace(i, &new.timers);
            self.timers.sort();
        }
        Ok(())
    }

    pub fn trigger(&mut self, line: &str) -> TriggerRequests {
        let mut requests = TriggerRequests::default();
        for (i, trigger) in self
            .triggers
            .iter_mut()
            .filter(|(_, t)| t.regex.is_match(line))
        {
            if trigger.one_shot {
                trigger.enabled = false;
            }
            if !trigger.text.is_empty() {
                requests.send.push(SendRequest {
                    send_to: trigger.send_to,
                    text: &trigger.text,
                    plugin: i,
                });
            }
            requests.hide |= trigger.omit_from_output;
            if !requests.hide {
                requests.make_bold |= trigger.make_bold;
                requests.make_italic |= trigger.make_italic;
                requests.make_underline |= trigger.make_underline;
                if trigger.change_foreground {
                    requests.foreground = Some(&trigger.foreground);
                }
                if trigger.change_background {
                    requests.background = Some(&trigger.background);
                }
            }

            if !trigger.keep_evaluating {
                return requests;
            }
        }
        requests
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
