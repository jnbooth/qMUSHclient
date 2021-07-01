use std::fmt::{self, Debug, Display, Formatter};
use std::os::raw::c_int;

use cpp_core::{CastFrom, CppBox};
use qt_core::QString;
use qt_gui::q_font::{Capitalization, Style, StyleHint};
use qt_gui::q_font_database::SystemFont;
use qt_gui::{QFont, QFontDatabase};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::Binding;

#[derive(Binding)]
pub struct RFont(CppBox<QFont>);

impl Debug for RFont {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RFont")
            .field("family", &self.family())
            .field("size", &self.size())
            //.field("weight", &self.weight())
            //.field("italic", &self.italic())
            .finish()
    }
}

impl Display for RFont {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} pt.", self.family(), self.size())
    }
}

impl CastFrom<&RFont> for cpp_core::Ref<QFont> {
    #[inline]
    unsafe fn cast_from(value: &RFont) -> Self {
        unsafe { value.0.as_ref() }
    }
}

impl Clone for RFont {
    fn clone(&self) -> Self {
        Self::from(unsafe { QFont::new_copy(&self.0) })
    }
}

impl Default for RFont {
    fn default() -> Self {
        Self::from(unsafe { QFont::new() })
    }
}

impl From<SystemFont> for RFont {
    fn from(value: SystemFont) -> Self {
        Self::from(unsafe { QFontDatabase::system_font(value) })
    }
}

impl RFont {
    pub fn global(hint: StyleHint) -> Self {
        unsafe {
            let font = QFont::new();
            font.set_style_hint_1a(hint);
            font.set_family(&font.default_family());
            Self::from(font)
        }
    }

    pub fn family(&self) -> String {
        unsafe { self.0.family() }.to_std_string()
    }
    pub fn set_family(&self, family: &str) {
        unsafe {
            self.0.set_family(&QString::from_std_str(family));
        }
    }

    pub fn size(&self) -> c_int {
        unsafe { self.0.point_size() }
    }
    pub fn set_size(&self, size: c_int) {
        unsafe {
            self.0.set_point_size(size);
        }
    }

    pub fn style_hint(&self) -> StyleHint {
        unsafe { self.0.style_hint() }
    }
    pub fn set_style_hint(&self, style_hint: StyleHint) {
        unsafe {
            let strategy = self.0.style_strategy();
            self.0.set_style_hint_2a(style_hint, strategy);
        }
    }

    qt_field!(weight, set_weight, c_int);

    qt_field!(style, set_style, Style);

    qt_field!(italic, set_italic, bool);

    qt_field!(underline, set_underline, bool);

    qt_field!(strike_out, set_strike_out, bool);

    qt_field!(fixed_pitch, set_fixed_pitch, bool);

    qt_field!(capitalization, set_capitalization, Capitalization);
}

impl<'de> Deserialize<'de> for RFont {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(|desc| unsafe {
            let font = QFont::new();
            font.from_string(&QString::from_std_str(&desc));
            Self::from(font)
        })
    }
}

impl Serialize for RFont {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        unsafe { self.0.to_string() }
            .to_std_string()
            .serialize(serializer)
    }
}
