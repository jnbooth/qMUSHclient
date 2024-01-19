use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::Write;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::Path;
use std::rc::{Rc, Weak};
use std::{io, mem, str};

use enumeration::EnumSet;
use mlua::{self, UserData, Value};
use qmushclient_scripting::{
    Alias, Callback, Event, LoadError, Pad, Plugin, PluginHandler, PluginIndex, PluginPack,
    ScriptArgs, ScriptRes, SendMatch, SendTo, Sendable, Timer, Trigger, TriggerEffects,
};
use qt::core::{QTimer, TimerKind};
use uuid::Uuid;

use super::method_gatherer::MethodGatherer;
use crate::api::Api;
use crate::ui::WorldTab;
use crate::world::World;

const API_KEY: &str = "__ud";

const fn truthy(value: &Value) -> bool {
    match *value {
        Value::Nil | Value::Boolean(false) | Value::Integer(0) => false,
        Value::Number(f) => f as u64 == 0,
        _ => true,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SendRequest<'a> {
    pub plugin: PluginIndex,
    pub line: &'a str,
    pub wildcards: Vec<&'a str>,
    pub pad: Option<Pad>,

    pub send_to: SendTo,
    pub label: String,
    pub script: String,
    pub variable: String,
    pub text: String,
    pub omit_from_output: bool,
}

trait PluginWithApi {
    fn with_api<F: FnOnce(&Api)>(&self, f: F);
    fn with_api_mut<F: FnOnce(&mut Api)>(&self, f: F);
}

impl PluginWithApi for Plugin {
    fn with_api<F: FnOnce(&Api)>(&self, f: F) {
        if let Err(e) = self.with_userdata(API_KEY, f) {
            Plugin::alert_error(&self.metadata, &e);
        }
    }

    fn with_api_mut<F: FnOnce(&mut Api)>(&self, f: F) {
        if let Err(e) = self.with_userdata_mut(API_KEY, f) {
            Plugin::alert_error(&self.metadata, &e);
        }
    }
}

pub struct PluginEngine {
    event_handler: Weak<WorldTab>,
    api: Api,
    initialize: String,
    plugins: Vec<Plugin>,
    timers: HashMap<Uuid, QTimer>,
}

impl PluginEngine {
    pub fn set_event_handler(&mut self, event_handler: Weak<WorldTab>) {
        self.event_handler = event_handler;
    }

    pub fn trigger_timer(&mut self, id: Uuid, request: SendRequest) -> io::Result<()> {
        if let Entry::Occupied(entry) = self.timers.entry(id) {
            if entry.get().kind() == TimerKind::Once {
                entry.remove();
            }
        }
        self.handle_send(request)
    }

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
            SendTo::WorldDelay => (),     // TODO
            SendTo::WorldImmediate => (), // TODO
            SendTo::Command => self.api.command(&request.text),
            SendTo::Output => self.api.note(&request.text),
            SendTo::Status => (), // TODO need to implement status bar first
            SendTo::NotepadNew => (),
            SendTo::NotepadAppend => self
                .api
                .notepad_mut()
                .append(request.pad.unwrap(), &request.text),
            SendTo::NotepadReplace => self
                .api
                .notepad_mut()
                .replace(request.pad.unwrap(), &request.text),
            SendTo::Log => (),       // TODO
            SendTo::Speedwalk => (), // TODO need to implement speedwalk system first
            SendTo::Execute => (),   // TODO
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
            SendTo::ScriptAfterOmit => (), // TODO
        };
        Ok(())
    }

    fn iter(&self, cb: Callback) -> impl Iterator<Item = &Plugin> {
        self.plugins
            .iter()
            .filter(move |p| p.callbacks.contains(cb))
    }

    fn init_plugin(&self, pack: PluginPack) -> mlua::Result<Plugin> {
        let engine = qmushclient_scripting::new_lua()?;

        engine.globals().set(
            API_KEY,
            self.api.clone_with(self.plugins.len(), &pack.metadata),
        )?;
        engine.load(&self.initialize).exec()?;
        Plugin::load(engine, pack)
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
        timers: &mut HashMap<Uuid, QTimer>,
        plugin_index: PluginIndex,
        plugin_name: String,
        timer: &Timer,
    ) {
        let duration = match timer.event {
            Event::Interval(duration) => duration,
            Event::Time(..) => return,
        };
        let kind = if timer.one_shot {
            TimerKind::Once
        } else {
            TimerKind::Repeating
        };
        let rtimer = QTimer::new(kind, duration);
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

    pub fn _send_to_first<A: ScriptArgs + Clone>(&self, cb: Callback, args: A) -> bool {
        self.iter(cb)
            .any(|p| p.call(cb, args.clone()).unwrap_or(false))
    }
}

impl PluginHandler for PluginEngine {
    type PluginWorld = World;
    type Userdata = Api;

    fn new(api: Api) -> Self {
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
            api,
            initialize,
            plugins: Vec::new(),
            timers: HashMap::new(),
        }
    }

    fn clear(&mut self) {
        self.plugins.clear();
    }

    fn load_plugin_file(&mut self, path: &Path) -> Result<(), LoadError> {
        let file = File::open(path)?;
        let pack = PluginPack::from_xml(BufReader::new(file))?;
        self.load_plugin(pack)?;
        Ok(())
    }

    fn load_plugin(&mut self, pack: PluginPack) -> mlua::Result<()> {
        self.plugins.push(self.init_plugin(pack)?);
        Ok(())
    }
    fn sort(&mut self) {
        let mut senders = self.api.senders_mut();
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

    fn alter_userdata<F: FnMut(&mut Api)>(&mut self, mut f: F) {
        f(&mut self.api);
        for plugin in &mut self.plugins {
            plugin.with_api_mut(&mut f);
        }
    }

    fn update_world_plugin(&mut self, old: &Rc<World>, new: &Rc<World>) -> mlua::Result<()> {
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
        let mut senders = self.api.senders_mut();
        if old.triggers != new.triggers {
            senders.replace_all(i, &new.triggers);
        }
        if old.aliases != new.aliases {
            senders.replace_all(i, &new.aliases);
        }
        if old.timers != new.timers {
            senders.replace_all(i, &new.timers);
        }
        Ok(())
    }

    fn alias(&mut self, line: &str) -> io::Result<bool> {
        let mut requests = Vec::new();
        let mut delete_oneshots = Vec::new();
        {
            let mut senders = self.api.senders_mut();
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

    fn trigger(&mut self, line: &str) -> io::Result<TriggerEffects> {
        let mut effects = TriggerEffects::default();
        let mut requests = Vec::new();
        let mut delete_oneshots = Vec::new();
        {
            let mut senders = self.api.senders_mut();
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

    fn receive_from_all<A>(&self, cb: Callback, mut args: A) -> A
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

    fn send_to_all<A: ScriptArgs + Clone>(&self, cb: Callback, args: A) {
        for plugin in self.iter(cb) {
            plugin.call(cb, args.clone()).unwrap_or(());
        }
    }

    fn send_to_all_until<A: ScriptArgs + Clone>(
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