use std::cmp::Ordering;
use std::{mem, str};

use enumeration::{Enum, EnumSet};
use mlua::{self, AnyUserData, FromLuaMulti, Function, Lua, Value};
use qt::widgets::{MessageBoxIcon, QMessageBox};
use tr::TrContext;

use super::{PluginMetadata, PluginPack};
use crate::callback::Callback;
use crate::convert::ScriptArgs;
use crate::send::{Alias, Timer, Trigger};

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
        Some(self.cmp(other))
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
            .filter(|cb| {
                matches!(
                    globals.raw_get(format!("{:?}", cb)),
                    Ok(Value::Function(..))
                )
            })
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

    pub fn with_userdata<T, R, F>(&self, key: &str, f: F) -> mlua::Result<R>
    where
        T: 'static,
        F: FnOnce(&T) -> R,
    {
        let global_ref = self.engine.globals().raw_get::<_, AnyUserData>(key)?;
        let global_val = global_ref.borrow()?;
        Ok(f(&global_val))
    }

    pub fn with_userdata_mut<T, R, F>(&self, key: &str, f: F) -> mlua::Result<R>
    where
        T: 'static,
        F: FnOnce(&mut T) -> R,
    {
        let global_ref = self.engine.globals().raw_get::<_, AnyUserData>(key)?;
        let mut global_val = global_ref.borrow_mut()?;
        Ok(f(&mut global_val))
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
        self.invoke(&format!("{:?}", cb), args)
    }
}
