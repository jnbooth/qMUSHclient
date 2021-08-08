macro_rules! impl_eq_cpp {
    ($t:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(unsafe { &other.0.as_ref() })
            }
        }

        impl Eq for $t {}
    };
}

macro_rules! qt_field {
    ($get:ident, $set:ident, $t:ty) => {
        pub fn $get(&self) -> $t {
            unsafe { self.0.$get() }
        }
        pub fn $set(&self, $get: $t) {
            unsafe { self.0.$set($get) }
        }
    };
}

pub mod color;
pub use color::RColor;

pub mod text;

mod font;
pub use font::RFont;

mod graphics;
pub use graphics::{RImage, Rect};

mod io;
pub use io::RIODevice;

mod list;
pub use list::QList;

mod printable;
pub use printable::Printable;

mod settings;
pub use settings::RSettings;

mod time;
pub use time::{RTimer, TimerKind};

pub mod variant;

mod widgets;
pub use widgets::{Browse, RForm, RWidget};
