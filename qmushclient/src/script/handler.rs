use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::rc::{Rc, Weak};
use std::{io, mem, str};

use enumeration::EnumSet;
use mlua::{self, FromLuaMulti, IntoLuaMulti, Lua, UserData, UserDataMethods, Value};
use qt::{RColor, RTimer, TimerKind};
use uuid::Uuid;

use super::callback::Callback;
use super::convert::{ScriptArgs, ScriptRes};
use super::PluginPack;
use crate::api::Api;
use crate::script::indexed::{PluginIndex, Senders};
use crate::script::plugin::API_KEY;
use crate::script::{Alias, Plugin, SendMatch, SendTo, Sendable, Timer, Trigger};
use crate::ui::{Pad, WorldTab};
use crate::world::World;

const fn truthy(value: &Value) -> bool {
    match *value {
        Value::Nil | Value::Boolean(false) | Value::Integer(0) => false,
        Value::Number(f) => f as u64 == 0,
        _ => true,
    }
}

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

#[derive(Debug, Error)]
pub enum LoadError {
    File(io::Error),
    Xml(quick_xml::DeError),
    Script(mlua::Error),
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendRequest<'a> {
    plugin: PluginIndex,
    line: &'a str,
    wildcards: Vec<&'a str>,
    pad: Option<Pad>,

    send_to: SendTo,
    label: String,
    script: String,
    variable: String,
    text: String,
    omit_from_output: bool,
}

#[derive(Clone, Debug, Default)]
pub struct TriggerEffects {
    pub hide: bool,
    pub foreground: Option<RColor>,
    pub background: Option<RColor>,
    pub sounds: Vec<PathBuf>,
    pub make_bold: bool,
    pub make_italic: bool,
    pub make_underline: bool,
}

pub struct PluginHandler {
    event_handler: Weak<WorldTab>,
    api: Api,
    initialize: String,
    plugins: Vec<Plugin>,
    senders: Rc<RefCell<Senders>>,
    timers: HashMap<Uuid, RTimer>,
}

impl PluginHandler {
    fn handle_send(&self, request: SendRequest) -> io::Result<()> {
        if !request.script.is_empty() {
            let _: mlua::Result<()> = self.plugins[request.plugin].invoke(
                &request.script,
                (&request.label, request.line, request.wildcards, []),
            );
        }
        if request.send_to.ignore_empty() && request.text.is_empty() {
            return Ok(());
        }
        match request.send_to {
            SendTo::World => {
                if !request.omit_from_output {
                    self.api.echo(&request.text);
                }
                self.api.send([&request.text, "\n"].concat())?;
            }
            SendTo::Command => self.api.command(&request.text),
            SendTo::Output => self.api.note(&request.text),
            SendTo::Status => (), // TODO need to implement status bar first
            SendTo::NotepadAppend => self
                .api
                .notepad
                .borrow_mut()
                .append(request.pad.unwrap(), &request.text),
            SendTo::NotepadReplace => self
                .api
                .notepad
                .borrow_mut()
                .replace(request.pad.unwrap(), &request.text),
            SendTo::Speedwalk => (), // TODO need to implement speedwalk system first
            SendTo::Variable => {
                if request.variable.is_empty() {
                    return Ok(());
                }
                let variable = request.variable;
                let text = request.text;
                self.plugins[request.plugin].with_api(move |api| api.set_variable(variable, text));
            }
            SendTo::Script => {
                let plugin = &self.plugins[request.plugin];
                if let Err(e) = plugin.engine.load(&request.text).exec() {
                    Plugin::alert_error(&plugin.metadata, &e);
                }
            }
        };
        Ok(())
    }

    pub fn new(api: Api) -> Self {
        let mut gatherer = MethodGatherer::new();
        Api::add_methods(&mut gatherer);

        let mut initialize = String::new();
        for me in gatherer.into_vec() {
            let _ = writeln!(
                initialize,
                "_G[{:?}] = function(...) __{}[{:?}](__{},...) end",
                me, API_KEY, me, API_KEY,
            );
        }

        Self {
            event_handler: Weak::new(),
            senders: api.senders.clone(),
            api,
            initialize,
            plugins: Vec::new(),
            timers: HashMap::new(),
        }
    }

    pub fn set_event_handler(&mut self, event_handler: Weak<WorldTab>) {
        self.event_handler = event_handler;
    }

    pub fn clear(&mut self) {
        self.plugins.clear();
    }

    fn iter(&self, cb: Callback) -> impl Iterator<Item = &Plugin> {
        self.plugins
            .iter()
            .filter(move |p| p.callbacks.contains(cb))
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

        engine.globals().set(
            API_KEY,
            self.api.clone_with(self.plugins.len(), &pack.metadata),
        )?;
        engine.load(&self.initialize).exec()?;
        Plugin::load(engine, pack)
    }

    pub fn sort(&mut self) {
        let mut senders = self.senders.borrow_mut();
        senders.clear();
        self.plugins.sort_unstable();
        self.timers.clear();

        for (i, plugin) in self.plugins.iter_mut().enumerate() {
            senders.extend(i, plugin);
            plugin.with_api_mut(|api| api.set_index(i));
            for timer in &plugin.timers {
                Self::start_timer(
                    self.event_handler.clone(),
                    &mut self.timers,
                    i,
                    plugin.metadata.name.clone(),
                    timer,
                );
            }
        }
    }

    pub fn alter_userdata<F: FnMut(&mut Api)>(&mut self, mut f: F) {
        f(&mut self.api);
        for plugin in &mut self.plugins {
            plugin.with_api_mut(&mut f);
        }
    }

    pub fn update_world_plugin(&mut self, old: &Rc<World>, new: &Rc<World>) -> mlua::Result<()> {
        let i = match self.plugins.iter().position(|x| x.metadata.is_world_plugin) {
            Some(i) => i,
            None => return self.load_plugin(new.world_plugin()),
        };
        if old.world_script != new.world_script {
            let old_plugin = &mut self.plugins[i];
            let pack = PluginPack {
                metadata: mem::take(&mut old_plugin.metadata),
                triggers: mem::take(&mut old_plugin.triggers),
                aliases: mem::take(&mut old_plugin.aliases),
                timers: mem::take(&mut old_plugin.timers),
                script: new.world_script.clone(),
            };
            self.plugins[i] = self.init_plugin(pack)?;
        }
        if old.triggers != new.triggers {
            self.senders.borrow_mut().replace_all(i, &new.triggers);
        }
        if old.aliases != new.aliases {
            self.senders.borrow_mut().replace_all(i, &new.aliases);
        }
        if old.timers != new.timers {
            self.senders.borrow_mut().replace_all(i, &new.timers);
        }
        Ok(())
    }

    fn pad_for_send<T: Sendable>(&self, send: &SendMatch<T>) -> Option<Pad> {
        if matches!(
            send.sender.as_ref().send_to,
            SendTo::NotepadAppend | SendTo::NotepadReplace //| SendTo::NotepadNew
        ) {
            Some(send.sender.pad(&self.plugins[send.plugin].metadata))
        } else {
            None
        }
    }

    fn start_timer(
        event_handler: Weak<WorldTab>,
        timers: &mut HashMap<Uuid, RTimer>,
        plugin_index: PluginIndex,
        plugin_name: String,
        timer: &Timer,
    ) {
        let duration = match timer.event {
            super::Event::Interval(duration) => duration,
            super::Event::Time(..) => return,
        };
        let kind = if timer.one_shot {
            TimerKind::Once
        } else {
            TimerKind::Repeating
        };
        let rtimer = RTimer::new(kind, duration);
        let id = Uuid::new_v4();
        let request = SendRequest {
            plugin: plugin_index,
            line: "",
            wildcards: Vec::new(),
            pad: Some(Pad::Timer {
                plugin: plugin_name,
                event: timer.event,
            }),
            send_to: timer.send_to,
            label: timer.label.clone(),
            script: timer.script.clone(),
            variable: timer.variable.clone(),
            text: timer.text.clone(),
            omit_from_output: timer.omit_from_output,
        };
        rtimer.connect(move || {
            if let Some(event_handler) = event_handler.upgrade() {
                event_handler.trigger_timer(id, request.clone());
            }
        });
        rtimer.start();
        timers.insert(id, rtimer);
    }

    pub fn trigger_timer(&mut self, id: Uuid, request: SendRequest) -> io::Result<()> {
        if let Entry::Occupied(entry) = self.timers.entry(id) {
            if entry.get().kind() == TimerKind::Once {
                entry.remove();
            }
        }
        self.handle_send(request)
    }

    pub fn alias(&mut self, line: &str) -> io::Result<bool> {
        let mut requests = Vec::new();
        let mut delete_oneshots = Vec::new();
        {
            let mut senders = self.senders.borrow_mut();
            for send in senders.matches::<Alias>(line) {
                let alias = &send.sender;
                if alias.one_shot && delete_oneshots.last() != Some(&send.pos) {
                    delete_oneshots.push(send.pos);
                }
                requests.push(SendRequest {
                    pad: self.pad_for_send(&send),
                    plugin: send.plugin,
                    line,
                    wildcards: send.wildcards,

                    send_to: alias.send_to,
                    label: alias.label.clone(),
                    script: alias.script.clone(),
                    variable: alias.variable.clone(),
                    text: send.text.into_owned(),
                    omit_from_output: alias.omit_from_output,
                });
                if !alias.keep_evaluating {
                    break;
                }
            }
            senders.delete_all::<Alias>(&delete_oneshots);
        }
        let any_requests = requests.is_empty();
        for request in requests {
            self.handle_send(request)?;
        }
        Ok(!any_requests)
    }

    pub fn trigger(&mut self, line: &str) -> io::Result<TriggerEffects> {
        let mut effects = TriggerEffects::default();
        let mut requests = Vec::new();
        let mut delete_oneshots = Vec::new();
        {
            let mut senders = self.senders.borrow_mut();
            for send in senders.matches::<Trigger>(line) {
                let trigger = &send.sender;
                if trigger.one_shot && delete_oneshots.last() != Some(&send.pos) {
                    delete_oneshots.push(send.pos);
                }
                let send_to = trigger.send_to;
                if !send_to.ignore_empty()
                    || !send.text.is_empty()
                    || !trigger.script.is_empty()
                    || (send_to == SendTo::Variable && !trigger.variable.is_empty())
                {
                    requests.push(SendRequest {
                        pad: self.pad_for_send(&send),
                        plugin: send.plugin,
                        line,
                        wildcards: send.wildcards,

                        send_to,
                        label: trigger.label.clone(),
                        script: trigger.script.clone(),
                        variable: trigger.variable.clone(),
                        text: send.text.into_owned(),
                        omit_from_output: trigger.omit_from_output,
                    });
                }

                effects.hide |= trigger.omit_from_output;
                if !effects.hide {
                    effects.make_bold |= trigger.make_bold;
                    effects.make_italic |= trigger.make_italic;
                    effects.make_underline |= trigger.make_underline;
                    if trigger.change_foreground {
                        effects.foreground = Some(trigger.foreground.clone());
                    }
                    if trigger.change_background {
                        effects.background = Some(trigger.background.clone());
                    }
                }

                if !trigger.keep_evaluating {
                    break;
                }
            }
            senders.delete_all::<Trigger>(&delete_oneshots);
        }

        for request in requests {
            self.handle_send(request)?;
        }

        Ok(effects)
    }

    pub fn _send_to_first<A: ScriptArgs + Clone>(&self, cb: Callback, args: A) -> bool {
        self.iter(cb)
            .any(|p| p.call(cb, args.clone()).unwrap_or(false))
    }

    pub fn receive_from_all<A>(&self, cb: Callback, mut args: A) -> A
    where
        A: 'static + ScriptArgs + ScriptRes + Clone,
    {
        for plugin in self.iter(cb) {
            if let Ok(new_args) = plugin.call(cb, args.clone()) {
                args = new_args;
            }
        }
        args
    }

    pub fn send_to_all<A: ScriptArgs + Clone>(&self, cb: Callback, args: A) {
        for plugin in self.iter(cb) {
            plugin.call(cb, args.clone()).unwrap_or(());
        }
    }

    pub fn send_to_all_until<A: ScriptArgs + Clone>(
        &self,
        cb: Callback,
        args: A,
        stop: EnumSet<bool>,
    ) -> bool {
        for plugin in self.iter(cb) {
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
