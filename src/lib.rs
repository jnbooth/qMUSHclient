#![allow(clippy::wrong_self_convention)]
#![allow(clippy::from_str_radix_10)]
#![warn(unsafe_op_in_unsafe_fn)]

#[macro_use]
extern crate qmushclient_derive;
#[macro_use]
extern crate enumeration;

extern crate libsqlite3_sys;
extern crate luajit_src;
extern crate pcre2_sys;

#[cfg(debug_assertions)]
#[macro_use]
mod rgbtrace;
#[cfg(not(debug_assertions))]
macro_rules! rgbtrace {
    ($($t:tt)*) => {};
}

//#[macro_use]
//pub mod enums;
#[macro_use]
pub mod tr;

mod api;
pub mod binding;
pub mod caseinsensitive;
mod client;
mod constants;
mod escape;
mod ffi;
mod mxp;
mod persist;
mod script;
mod ui;
mod world;

pub use binding::Widget;
pub use ui::App;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(u32);

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:01}.{:02}", self.0 / 100, self.0 % 100)
    }
}

pub trait DurationExt {
    fn hour(&self) -> u64;
    fn minute(&self) -> u64;
    fn second(&self) -> f64;
    fn milli(&self) -> u32;
    fn from_hms(hour: u64, minute: u64, second: f64) -> Self;
}

const NANOS: u64 = 1_000_000_000;

impl DurationExt for std::time::Duration {
    fn hour(&self) -> u64 {
        self.as_secs() / 3600
    }

    fn minute(&self) -> u64 {
        (self.as_secs() % 3600) / 3600
    }

    fn second(&self) -> f64 {
        self.subsec_nanos() as f64 / NANOS as f64
    }

    fn milli(&self) -> u32 {
        self.subsec_millis()
    }

    fn from_hms(hour: u64, minute: u64, second: f64) -> Self {
        Self::from_nanos((NANOS as f64 * second) as u64 + NANOS * 60 * (minute + 60 * hour))
    }
}
