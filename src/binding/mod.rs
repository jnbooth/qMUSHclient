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

mod image;
pub use image::RImage;

mod io;
pub use io::RIODevice;

mod list;
pub use list::QList;

mod printable;
pub use printable::Printable;

mod settings;
pub use settings::RSettings;

pub mod variant;

mod widgets;
pub use widgets::{Browse, RDialog, RForm, RWidget};
