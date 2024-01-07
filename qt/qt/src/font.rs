use std::borrow::Cow;
use std::fmt::{self, Debug, Display, Formatter};
use std::os::raw::c_int;

use cpp_core::{CastFrom, CppBox};
use qt_core::QString;
use qt_gui as q;
use qt_gui::q_font::{Capitalization, Style, StyleHint, Weight};
use qt_gui::q_font_database::SystemFont;
#[cfg(feature = "serde")]
use serde::de::{Error as _, Unexpected};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const fn display_style(style: Style) -> &'static str {
    match style {
        Style::StyleItalic => "italic ",
        Style::StyleOblique => "oblique ",
        _ => "",
    }
}

const fn display_variant(capit: Capitalization) -> &'static str {
    match capit {
        Capitalization::SmallCaps => "small-caps ",
        _ => "",
    }
}

const fn display_weight(weight: Weight) -> &'static str {
    match weight {
        Weight::Thin => "100 ",
        Weight::ExtraLight => "200 ",
        Weight::Light => "300 ",
        Weight::Normal => "400 ",
        Weight::Medium => "500 ",
        Weight::DemiBold => "600 ",
        Weight::Bold => "700 ",
        Weight::ExtraBold => "800 ",
        Weight::Black => "900 ",
        _ => "",
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QFont {
    pub(crate) inner: CppBox<q::QFont>,
}

impl_eq_cpp!(QFont);

impl From<CppBox<q::QFont>> for QFont {
    fn from(value: CppBox<q::QFont>) -> Self {
        Self { inner: value }
    }
}

impl Display for QFont {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{style}{variant}{weight}{size}px {family:?}",
            style = display_style(self.style()),
            variant = display_variant(self.capitalization()),
            weight = display_weight(self.weight()),
            size = self.size(),
            family = self.family(),
        )
    }
}

impl CastFrom<&QFont> for cpp_core::Ref<q::QFont> {
    #[inline]
    unsafe fn cast_from(value: &QFont) -> Self {
        unsafe { value.inner.as_ref() }
    }
}

impl Clone for QFont {
    fn clone(&self) -> Self {
        Self::from(unsafe { q::QFont::new_copy(&self.inner) })
    }
}

impl Default for QFont {
    fn default() -> Self {
        Self::from(unsafe { q::QFont::new() })
    }
}

impl From<SystemFont> for QFont {
    fn from(value: SystemFont) -> Self {
        Self::from(unsafe { q::QFontDatabase::system_font(value) })
    }
}

impl QFont {
    #[cfg(not(test))]
    pub fn global(hint: StyleHint) -> Self {
        unsafe {
            let font = q::QFont::new();
            font.set_style_hint_1a(hint);
            font.set_family(&font.default_family());
            Self::from(font)
        }
    }
    #[cfg(test)] // font database may not be initialized
    pub fn global(_: StyleHint) -> Self {
        Self::default()
    }

    pub fn family(&self) -> String {
        unsafe { self.inner.family().to_std_string() }
    }
    pub fn set_family(&self, family: &str) {
        unsafe {
            self.inner.set_family(&QString::from_std_str(family));
        }
    }

    pub fn size(&self) -> c_int {
        unsafe { self.inner.point_size() }
    }
    pub fn set_size(&self, size: c_int) {
        unsafe {
            self.inner.set_point_size(size);
        }
    }

    pub fn style_hint(&self) -> StyleHint {
        unsafe { self.inner.style_hint() }
    }
    pub fn set_style_hint(&self, style_hint: StyleHint) {
        unsafe {
            let strategy = self.inner.style_strategy();
            self.inner.set_style_hint_2a(style_hint, strategy);
        }
    }

    pub fn weight(&self) -> Weight {
        Weight::from_int(unsafe { self.inner.weight() })
    }

    pub fn set_weight(&self, weight: Weight) {
        unsafe { self.inner.set_weight(weight.to_int()) }
    }

    pub fn metrics(&self) -> QFontMetrics {
        QFontMetrics::new(unsafe { q::QFontMetrics::new_1a(&self.inner) })
    }

    qt_field!(style, set_style, Style);

    qt_field!(italic, set_italic, bool);

    qt_field!(underline, set_underline, bool);

    qt_field!(strike_out, set_strike_out, bool);

    qt_field!(fixed_pitch, set_fixed_pitch, bool);

    qt_field!(capitalization, set_capitalization, Capitalization);
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for QFont {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = Cow::<'de, str>::deserialize(deserializer)?;
        if s.starts_with(',') {
            // means it's an unspecified font
            Ok(Self::default())
        } else {
            unsafe {
                let font = q::QFont::new();
                if font.from_string(&QString::from_std_str(&s)) {
                    Ok(Self::from(font))
                } else {
                    Err(D::Error::invalid_value(
                        Unexpected::Str(&s),
                        &"valid QFont specifier",
                    ))
                }
            }
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for QFont {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        unsafe { self.inner.to_string() }
            .to_std_string()
            .serialize(serializer)
    }
}

pub struct QFontMetrics {
    pub(crate) inner: CppBox<q::QFontMetrics>,
}

impl_eq_cpp!(QFontMetrics);

impl Clone for QFontMetrics {
    fn clone(&self) -> Self {
        unsafe {
            Self {
                inner: q::QFontMetrics::new_copy(&self.inner),
            }
        }
    }
}

impl QFontMetrics {
    pub fn new(inner: CppBox<q::QFontMetrics>) -> Self {
        Self { inner }
    }

    pub fn average_char_width(&self) -> c_int {
        unsafe { self.inner.average_char_width() }
    }

    pub fn height(&self) -> c_int {
        unsafe { self.inner.height() }
    }
}
