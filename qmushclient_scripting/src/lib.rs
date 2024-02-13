#![warn(unsafe_op_in_unsafe_fn)]
#![allow(clippy::result_large_err)]

extern crate enumeration;
#[macro_use]
extern crate enumeration_derive;
#[macro_use]
extern crate tr;
#[macro_use]
extern crate tr_derive;

extern crate lua_modules_sys;
pub use lua_modules_sys::{SQLITE_VERSION, SQLITE_VERSION_NUMBER};

#[macro_use]
mod in_place;

mod callback;
pub use callback::Callback;

mod constants;

mod convert;
pub use convert::{ScriptArg, ScriptArgs, ScriptRes};

mod ffi;
pub use ffi::new_lua;

mod plugins;
pub use plugins::{
    LoadError, Pad, Plugin, PluginHandler, PluginIndex, PluginMetadata, PluginPack, SendMatch,
    Sendable, Senders, TriggerEffects,
};

mod regex;
pub use regex::{Regex, RegexError};

mod send;
pub use send::{Alias, Event, Reaction, SendTo, Sender, Timer, Trigger};
