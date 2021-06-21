#![allow(dead_code)] // TODO

#[macro_use]
extern crate qmushclient_derive;

#[macro_use]
mod enums;
#[macro_use]
mod tr;

mod api;
mod binding;
mod caseinsensitive;
mod client;
mod constants;
mod escape;
mod mxp;
mod persist;
mod prependbufreader;
mod script;
mod ui;
mod world;

pub use binding::RWidget;
pub use ui::App;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:01}.{:02}", self.0 / 100, self.0 % 100)
    }
}
