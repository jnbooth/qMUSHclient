use std::cmp::Ordering;
use std::{mem, str};

use enumeration::{Enum, EnumSet};
use mlua::{self, AnyUserData, FromLuaMulti, Function, Lua, Value};
use qt::widgets::{MessageBoxIcon, QMessageBox};
use tr::TrContext;

use super::callback::Callback;
use super::convert::ScriptArgs;
use super::{PluginMetadata, PluginPack};
use crate::api::Api;
use crate::script::{Alias, Timer, Trigger};

pub const API_KEY: &str = "__ud";

#[derive(TrContext)]
pub struct Plugin {
    pub metadata: PluginMetadata,
    /// Which callbacks it responds to.
    pub callbacks: EnumSet<Callback>,
    /// Script engine for script.
    pub engine: Lua,
    pub triggers: Vec<Trigger>,
    pub aliases: Vec<Alias>,
    pub timers: Vec<Timer>,
}

impl PartialEq for Plugin {
    fn eq(&self, other: &Self) -> bool {
        self.metadata.eq(&other.metadata)
    }
}

impl Eq for Plugin {}

impl PartialOrd for Plugin {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.metadata.partial_cmp(&other.metadata)
    }
}

impl Ord for Plugin {
    fn cmp(&self, other: &Self) -> Ordering {
        self.metadata.cmp(&other.metadata)
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
            callbacks,
            engine,
            triggers: pack.triggers,
            aliases: pack.aliases,
            timers: pack.timers,
        })
    }

    pub fn alert_error(metadata: &PluginMetadata, error: &mlua::Error) {
        let msgbox = QMessageBox::new(());
        msgbox.set_icon(MessageBoxIcon::Warning);
        msgbox.set_text(tr!("Script error in {}", metadata.name));
        msgbox.set_informative_text(error.to_string());
        msgbox.exec();
    }

    pub fn with_api<F: FnOnce(&Api)>(&self, f: F) {
        if let Ok(apiref) = self.engine.globals().raw_get::<_, AnyUserData>(API_KEY) {
            if let Ok(api) = apiref.borrow() {
                f(&api);
            }
        }
    }

    pub fn with_api_mut<F: FnOnce(&mut Api)>(&self, f: F) {
        if let Ok(apiref) = self.engine.globals().raw_get::<_, AnyUserData>(API_KEY) {
            if let Ok(mut api) = apiref.borrow_mut() {
                f(&mut api);
            }
        }
    }

    fn try_invoke<'lua, A, R>(&'lua self, fn_name: &str, args: A) -> mlua::Result<R>
    where
        A: ScriptArgs,
        R: FromLuaMulti<'lua>,
    {
        let f: Function<'lua> = self.engine.globals().raw_get(fn_name)?;
        f.call(args.to_args(&self.engine)?)
    }

    pub fn invoke<'lua, A, R>(&'lua self, fn_name: &str, args: A) -> mlua::Result<R>
    where
        A: ScriptArgs,
        R: FromLuaMulti<'lua>,
    {
        let res = self.try_invoke(fn_name, args);
        if let Err(e) = &res {
            Self::alert_error(&self.metadata, e);
        }
        res
    }

    pub fn call<'lua, A, R>(&'lua self, cb: Callback, args: A) -> mlua::Result<R>
    where
        A: ScriptArgs,
        R: FromLuaMulti<'lua>,
    {
        self.invoke(cb.to_str(), args)
    }
}
