macro_rules! impl_eq_cpp {
    ($t:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.inner.eq(unsafe { &other.inner.as_ref() })
            }
        }

        impl Eq for $t {}
    };
}

macro_rules! qt_field {
    ($get:ident, $set:ident, $t:ty) => {
        pub fn $get(&self) -> $t {
            unsafe { self.inner.$get() }
        }
        pub fn $set(&self, $get: $t) {
            unsafe { self.inner.$set($get) }
        }
    };
}

pub mod color;
pub use color::RColor;

mod font;
pub use font::RFont;

mod graphics;
pub use graphics::{RImage, RRect, RRectF};

mod io;
pub use io::RIODevice;

mod list;
pub use list::QList;

mod printable;
pub use printable::Printable;

mod settings;
pub use settings::RSettings;

pub mod text;

mod time;
pub use time::{RTimer, TimerKind};

pub mod variant;

mod traits;
pub use traits::{Browse, RForm, Widget};

pub mod widgets;
