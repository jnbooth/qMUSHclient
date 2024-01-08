use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::{io, str};

use enumeration::EnumSet;
use mlua;
use qt::gui::QColor;

use super::callback::Callback;
use super::convert::{ScriptArgs, ScriptRes};
use super::PluginPack;

#[derive(Debug, Error)]
pub enum LoadError {
    File(io::Error),
    Xml(quick_xml::DeError),
    Script(mlua::Error),
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
    type PluginApi;
    type PluginWorld;

    fn new(api: Self::PluginApi) -> Self;

    fn clear(&mut self);

    fn load_plugin_file(&mut self, path: &Path) -> Result<(), LoadError>;

    fn load_plugin(&mut self, pack: PluginPack) -> mlua::Result<()>;

    fn sort(&mut self);

    fn alter_userdata<F: FnMut(&mut Self::PluginApi)>(&mut self, f: F);

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
