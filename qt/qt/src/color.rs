use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::os::raw::{c_int, c_uint};

use cpp_core::{CastInto, CppBox, Ptr, Ref};
use qt_core::{GlobalColor, QString};
use qt_gui as q;
use qt_gui::q_palette::ColorRole;
use qt_widgets::q_color_dialog::ColorDialogOption;
use qt_widgets::q_dialog::DialogCode;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct QColor {
    pub(crate) inner: CppBox<q::QBrush>,
    code: c_uint,
    transparent: bool,
}

// SAFETY: QColor is immutable.
unsafe impl Send for QColor {}
unsafe impl Sync for QColor {}

impl PartialEq for QColor {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}
impl Eq for QColor {}
impl PartialOrd for QColor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(&other.code)
    }
}
impl Ord for QColor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(&other.code)
    }
}
impl Hash for QColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
impl Clone for QColor {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe { q::QBrush::new_copy(&self.inner) },
            code: self.code,
            transparent: self.transparent,
        }
    }
}
impl Debug for QColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_struct("RColor")
            .field("code", &format_args!("#{:08X}", self.code))
            .field("transparent", &self.transparent)
            .finish()
    }
}
impl Display for QColor {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "#{:08X}", self.code)
    }
}

impl From<CppBox<q::QColor>> for QColor {
    fn from(value: CppBox<q::QColor>) -> Self {
        Self::of(&value)
    }
}
impl From<CppBox<q::QBrush>> for QColor {
    fn from(value: CppBox<q::QBrush>) -> Self {
        Self::of(unsafe { value.color() })
    }
}
impl From<ColorRole> for QColor {
    fn from(value: ColorRole) -> Self {
        Self::of(unsafe { q::QGuiApplication::palette().color_1a(value) })
    }
}

impl From<GlobalColor> for QColor {
    fn from(value: GlobalColor) -> Self {
        Self::of(unsafe { &q::QColor::from_global_color(value) })
    }
}

impl From<c_uint> for QColor {
    fn from(value: c_uint) -> Self {
        Self::from_code(value | 0xFF000000)
    }
}

impl Default for QColor {
    fn default() -> Self {
        Self::from(unsafe { q::QBrush::new() })
    }
}

impl QColor {
    fn color(&self) -> Ref<q::QColor> {
        unsafe { self.inner.color() }
    }

    pub fn red(&self) -> u8 {
        unsafe { self.color().red() as u8 }
    }
    pub fn green(&self) -> u8 {
        unsafe { self.color().green() as u8 }
    }
    pub fn blue(&self) -> u8 {
        unsafe { self.color().blue() as u8 }
    }
    pub fn alpha(&self) -> u8 {
        unsafe { self.color().alpha() as u8 }
    }

    pub fn of<R: CastInto<Ref<q::QColor>>>(color: R) -> Self {
        unsafe {
            let color = color.cast_into();
            Self {
                code: color.rgba(),
                transparent: color.alpha() == 0,
                inner: q::QBrush::from_q_color(color),
            }
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from(unsafe { q::QColor::from_rgb_3a(r as c_int, g as c_int, b as c_int) })
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from(unsafe { q::QColor::from_rgb_4a(r as c_int, g as c_int, b as c_int, a as c_int) })
    }
    pub fn from_code(code: c_uint) -> Self {
        Self::from(unsafe { q::QColor::from_rgba(code) })
    }
    pub fn code(&self) -> c_uint {
        self.code
    }
    pub fn named(name: &str) -> Option<Self> {
        unsafe {
            let color = q::QColor::from_q_string(&QString::from_std_str(name));
            if color.is_valid() {
                Some(Self::from(color))
            } else {
                None
            }
        }
    }

    fn with_hsla<F>(&self, adjust: F) -> Self
    where
        F: FnOnce(&mut c_int, &mut c_int, &mut c_int, &mut c_int),
    {
        let mut h = 0;
        let mut s = 0;
        let mut l = 0;
        let mut a = 0;
        unsafe {
            self.color()
                .to_hsl()
                .get_hsl_4a(&mut h, &mut s, &mut l, &mut a);
        }
        adjust(&mut h, &mut s, &mut l, &mut a);
        Self::from(unsafe { q::QColor::from_hsl_4a(h, s, l, a) })
    }

    pub fn reshade(&self, adjust: c_int) -> Self {
        self.with_hsla(|_h, _s, l, _a| *l = (*l + adjust).clamp(0, 255))
    }

    pub fn saturate(&self, adjust: c_int) -> Self {
        self.with_hsla(|_h, s, _l, _a| *s = (*s + adjust).clamp(0, 255))
    }

    pub fn invert(&self) -> Self {
        self.with_hsla(|h, _s, l, _a| {
            *h = (*h + 180) % 360;
            *l = 255 - *l;
        })
    }

    pub fn pick<P: CastInto<Ptr<qt_widgets::QWidget>>>(&self, parent: P) -> Option<Self> {
        unsafe {
            let dlg = qt_widgets::QColorDialog::from_q_color_q_widget(self.color(), parent);
            dlg.set_option_2a(ColorDialogOption::ShowAlphaChannel, true);
            if dlg.exec() == DialogCode::Accepted.to_int() {
                Some(Self::from(dlg.selected_color()))
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg(feature = "serde")]
#[derive(Serialize, Deserialize)]
pub struct QColorPair {
    pub foreground: QColor,
    pub background: QColor,
}

impl QColorPair {
    pub fn new<Fg: Into<QColor>, Bg: Into<QColor>>(foreground: Fg, background: Bg) -> Self {
        Self {
            foreground: foreground.into(),
            background: background.into(),
        }
    }

    pub fn invert(&mut self) {
        mem::swap(&mut self.foreground, &mut self.background);
    }

    pub fn stylesheet(&self) -> CppBox<QString> {
        QString::from_std_str(format!(
            "color: {}; background-color: {};",
            self.foreground, self.background
        ))
    }
}

pub trait HasPalette {
    fn palette_color(&self, role: ColorRole) -> QColor;
    fn set_palette_color(&self, role: ColorRole, color: &QColor);
}

impl HasPalette for qt_widgets::QWidget {
    fn palette_color(&self, role: ColorRole) -> QColor {
        QColor::of(unsafe { self.palette().color_1a(role) })
    }

    fn set_palette_color(&self, role: ColorRole, color: &QColor) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(role, &color.inner);
            self.set_palette(palette);
            self.repaint();
        }
    }
}

pub trait Colored {
    fn foreground_color(&self) -> QColor;
    fn set_foreground_color(&self, color: &QColor);
    fn background_color(&self) -> QColor;
    fn set_background_color(&self, color: &QColor);

    fn colors(&self) -> QColorPair {
        QColorPair {
            foreground: self.foreground_color(),
            background: self.background_color(),
        }
    }
    fn set_colors(&self, colors: &QColorPair) {
        self.set_foreground_color(&colors.foreground);
        self.set_background_color(&colors.background);
    }
}

impl Colored for q::QTextFormat {
    fn foreground_color(&self) -> QColor {
        QColor::from(unsafe { self.foreground() })
    }
    fn background_color(&self) -> QColor {
        QColor::from(unsafe { self.background() })
    }
    fn set_foreground_color(&self, color: &QColor) {
        unsafe {
            self.set_foreground(&color.inner);
        }
    }
    fn set_background_color(&self, color: &QColor) {
        unsafe {
            if color.transparent {
                self.clear_background();
            } else {
                self.set_background(&color.inner);
            }
        }
    }
}

impl Colored for qt_widgets::QTextEdit {
    fn foreground_color(&self) -> QColor {
        unsafe { QColor::from(self.text_color()) }
    }
    fn background_color(&self) -> QColor {
        self.palette_color(ColorRole::Base)
    }
    fn set_foreground_color(&self, color: &QColor) {
        unsafe {
            self.set_text_color(color.color());
        }
    }
    fn set_background_color(&self, color: &QColor) {
        self.set_palette_color(ColorRole::Base, color);
    }
}

impl Colored for qt_widgets::QWidget {
    fn foreground_color(&self) -> QColor {
        self.palette_color(unsafe { self.foreground_role() })
    }
    fn background_color(&self) -> QColor {
        self.palette_color(unsafe { self.background_role() })
    }
    fn set_foreground_color(&self, color: &QColor) {
        self.set_palette_color(unsafe { self.foreground_role() }, color);
    }
    fn set_background_color(&self, color: &QColor) {
        self.set_palette_color(unsafe { self.background_role() }, color);
    }
    fn set_colors(&self, colors: &QColorPair) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(self.foreground_role(), &colors.foreground.inner);
            palette.set_brush_2a(self.background_role(), &colors.background.inner);
            self.set_palette(palette);
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for QColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_code(c_uint::deserialize(deserializer)?))
    }
}

#[cfg(feature = "serde")]
impl Serialize for QColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.code.serialize(serializer)
    }
}
