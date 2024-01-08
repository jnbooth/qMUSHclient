use std::convert::Infallible;
use std::fmt::{self, Display, Formatter};
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{io, str};

use enumeration::EnumSet;
use mlua;
use qt::gui::QColor;

use super::file::PluginPack;
use crate::callback::Callback;
use crate::convert::{ScriptArgs, ScriptRes};

#[derive(Debug)]
pub enum LoadError {
    File(io::Error),
    Xml(quick_xml::DeError),
    Script(mlua::Error),
}

impl Display for LoadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(e) => Display::fmt(&e, f),
            Self::Xml(e) => Display::fmt(&e, f),
            Self::Script(e) => Display::fmt(&e, f),
        }
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::File(e) => Some(e),
            Self::Xml(e) => Some(e),
            Self::Script(e) => Some(e),
        }
    }
}

impl From<Infallible> for LoadError {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

impl From<io::Error> for LoadError {
    fn from(e: io::Error) -> Self {
        Self::File(e)
    }
}

impl From<quick_xml::DeError> for LoadError {
    fn from(e: quick_xml::DeError) -> Self {
        Self::Xml(e)
    }
}

impl From<mlua::Error> for LoadError {
    fn from(e: mlua::Error) -> Self {
        Self::Script(e)
    }
}

#[derive(Clone, Debug, Default)]
pub struct TriggerEffects {
    pub hide: bool,
    pub foreground: Option<QColor>,
    pub background: Option<QColor>,
    pub sounds: Vec<PathBuf>,
    pub make_bold: bool,
    pub make_italic: bool,
    pub make_underline: bool,
}

pub trait PluginHandler {
    type PluginWorld;
    type Userdata;

    fn new(api: Self::Userdata) -> Self;

    fn clear(&mut self);

    fn load_plugin_file(&mut self, path: &Path) -> Result<(), LoadError>;

    fn load_plugin(&mut self, pack: PluginPack) -> mlua::Result<()>;

    fn sort(&mut self);

    fn alter_userdata<F: FnMut(&mut Self::Userdata)>(&mut self, f: F);

    fn update_world_plugin(
        &mut self,
        old: &Rc<Self::PluginWorld>,
        new: &Rc<Self::PluginWorld>,
    ) -> mlua::Result<()>;

    fn alias(&mut self, line: &str) -> io::Result<bool>;

    fn trigger(&mut self, line: &str) -> io::Result<TriggerEffects>;

    fn receive_from_all<A>(&self, cb: Callback, args: A) -> A
    where
        A: 'static + ScriptArgs + ScriptRes + Clone;

    fn send_to_all<A: ScriptArgs + Clone>(&self, cb: Callback, args: A);

    fn send_to_all_until<A: ScriptArgs + Clone>(
        &self,
        cb: Callback,
        args: A,
        stop: EnumSet<bool>,
    ) -> bool;
}
