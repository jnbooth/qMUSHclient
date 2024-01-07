#![warn(unsafe_op_in_unsafe_fn)]
#[macro_use]
extern crate enumeration;
#[macro_use]
extern crate enumeration_derive;

pub use qt_core::{ApplicationAttribute, GlobalColor, Key, KeyboardModifier, MouseButton};

macro_rules! impl_deref_binding {
    ($t:ty, $inner:ty) => {
        impl std::ops::Deref for $t {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                Self::Target::cast(&self.inner)
            }
        }
    };
}

macro_rules! impl_eq_cpp {
    ($t:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                unsafe { self.inner.eq(&other.inner.as_ref()) }
            }
        }

        impl Eq for $t {}
    };
}

macro_rules! qt_binding {
    ($t:ident, $inner:ty) => {
        #[repr(transparent)]
        pub struct $t {
            #[allow(dead_code)]
            pub(crate) inner: $inner,
        }

        impl $t {
            pub(crate) fn cast(inner: &$inner) -> &Self {
                // SAFETY: #[repr(transparent)]
                unsafe { &*(inner as *const $inner as *const Self) }
            }
        }
    };

    ($t:ident, $inner:ty, $inherit:ty) => {
        qt_binding!($t, $inner);

        impl std::ops::Deref for $t {
            type Target = $inherit;

            fn deref(&self) -> &Self::Target {
                Self::Target::cast(&self.inner)
            }
        }
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

mod application;
pub use application::{QApplication, QCoreApplication, QGuiApplication};

mod color;
pub use color::{Colored, HasPalette, QColor, QColorPair};

mod font;
pub use font::QFont;

mod graphics;
pub use graphics::QImage;

mod io;
pub use io::QIODevice;

mod list;
pub use list::QList;

mod locale;
pub use locale::{QLocale, QTranslator};

mod object;

mod printable;
pub use printable::Printable;

mod settings;
pub use settings::QSettings;

mod shapes;
pub use shapes::{QRect, QRectF};

pub mod text;
pub use text::{
    QScrollBar, QTextBlock, QTextBlockFormat, QTextCharFormat, QTextCursor, QTextDocument,
    QTextFormat, QTextFragment, QTextFrame, QTextFrameFormat, QTextImageFormat, QTextLayout,
    QTextList, QTextListFormat, QTextTable, QTextTableFormat,
};

mod time;
pub use time::{QTimer, TimerKind};

pub mod variant;
pub use variant::QVariant;

mod traits;
pub use traits::{Browse, QForm, Widget};

mod widgets;
pub use widgets::{QAbstractScrollArea, QDialog, QFrame, QLineEdit, QMessageBox, QTextBrowser};
