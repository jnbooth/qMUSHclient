use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::mem;
use std::os::raw::{c_int, c_uint};

use cpp_core::{CastInto, CppBox, Ptr, Ref};
use qt_core::{GlobalColor, QString};
use qt_gui::q_palette::ColorRole;
use qt_gui::{QBrush, QColor, QGuiApplication, QTextFormat};
use qt_widgets::q_color_dialog::ColorDialogOption;
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::{QColorDialog, QTextEdit, QWidget};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub struct RColor {
    inner: CppBox<QBrush>,
    code: c_uint,
}
// SAFETY: RColor is immutable.
unsafe impl Send for RColor {}
unsafe impl Sync for RColor {}

impl PartialEq for RColor {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code
    }
}
impl Eq for RColor {}
impl PartialOrd for RColor {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.code.partial_cmp(&other.code)
    }
}
impl Ord for RColor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.code.cmp(&other.code)
    }
}
impl Hash for RColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}
impl Clone for RColor {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe { QBrush::new_copy(&self.inner) },
            code: self.code,
        }
    }
}
impl Debug for RColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("RColor")
            .field("code", &format_args!("#{:08X}", self.code))
            .finish()
    }
}
impl Display for RColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "#{:08X}", self.code)
    }
}

impl From<CppBox<QColor>> for RColor {
    fn from(value: CppBox<QColor>) -> Self {
        Self::of(&value)
    }
}
impl From<CppBox<QBrush>> for RColor {
    fn from(value: CppBox<QBrush>) -> Self {
        Self::of(unsafe { value.color() })
    }
}
impl From<ColorRole> for RColor {
    fn from(value: ColorRole) -> Self {
        Self::of(unsafe { QGuiApplication::palette().color_1a(value) })
    }
}

impl From<GlobalColor> for RColor {
    fn from(value: GlobalColor) -> Self {
        Self::of(unsafe { &QColor::from_global_color(value) })
    }
}
impl Default for RColor {
    fn default() -> Self {
        Self::from(unsafe { QBrush::new() })
    }
}

impl RColor {
    fn color(&self) -> Ref<QColor> {
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

    pub fn of<R: CastInto<Ref<QColor>>>(color: R) -> Self {
        unsafe {
            let color = color.cast_into();
            Self {
                code: color.rgba(),
                inner: QBrush::from_q_color(color),
            }
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::from(unsafe { QColor::from_rgb_3a(r as c_int, g as c_int, b as c_int) })
    }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self::from(unsafe { QColor::from_rgb_4a(r as c_int, g as c_int, b as c_int, a as c_int) })
    }
    pub fn from_code(code: c_uint) -> Self {
        Self::from(unsafe { QColor::from_rgba(code) })
    }
    pub fn named(name: &str) -> Option<Self> {
        unsafe {
            let color = QColor::from_q_string(&QString::from_std_str(name));
            if color.is_valid() {
                Some(Self::from(color))
            } else {
                None
            }
        }
    }

    pub fn reshade(&self, adjust: c_int) -> Self {
        unsafe {
            let mut h = 0;
            let mut s = 0;
            let mut l = 0;
            let mut a = 0;
            self.color()
                .to_hsl()
                .get_hsl_4a(&mut h, &mut s, &mut l, &mut a);
            Self::from(QColor::from_hsl_4a(h, s, (l + adjust).clamp(0, 359), a))
        }
    }

    pub fn pick<P: CastInto<Ptr<QWidget>>>(&self, parent: P) -> Option<Self> {
        unsafe {
            let dlg = QColorDialog::from_q_color_q_widget(self.color(), parent);
            dlg.set_option_2a(ColorDialogOption::ShowAlphaChannel, true);
            if dlg.exec() == DialogCode::Accepted.to_int() {
                Some(Self::from(dlg.selected_color()))
            } else {
                None
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RColorPair {
    pub foreground: RColor,
    pub background: RColor,
}

impl RColorPair {
    pub fn new<Fg: Into<RColor>, Bg: Into<RColor>>(foreground: Fg, background: Bg) -> Self {
        Self {
            foreground: foreground.into(),
            background: background.into(),
        }
    }

    pub fn invert(&mut self) {
        mem::swap(&mut self.foreground, &mut self.background);
    }

    pub fn stylesheet(&self) -> CppBox<QString> {
        QString::from_std_str(&format!(
            "* {{ color: {}; background-color: {}; }}",
            self.foreground, self.background
        ))
    }
}

pub trait HasPalette {
    fn palette_color(&self, role: ColorRole) -> RColor;
    fn set_palette_color(&self, role: ColorRole, color: &RColor);
}

impl HasPalette for QWidget {
    fn palette_color(&self, role: ColorRole) -> RColor {
        RColor::of(unsafe { self.palette().color_1a(role) })
    }

    fn set_palette_color(&self, role: ColorRole, color: &RColor) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(role, &color.inner);
            self.set_palette(palette);
        }
    }
}

pub trait Colored {
    fn foreground_color(&self) -> RColor;
    fn set_foreground_color(&self, color: &RColor);
    fn background_color(&self) -> RColor;
    fn set_background_color(&self, color: &RColor);

    fn colors(&self) -> RColorPair {
        RColorPair {
            foreground: self.foreground_color(),
            background: self.background_color(),
        }
    }
    fn set_colors(&self, colors: &RColorPair) {
        self.set_foreground_color(&colors.foreground);
        self.set_background_color(&colors.background);
    }
}

impl Colored for QTextFormat {
    fn foreground_color(&self) -> RColor {
        RColor::from(unsafe { self.foreground() })
    }
    fn background_color(&self) -> RColor {
        RColor::from(unsafe { self.background() })
    }
    fn set_foreground_color(&self, color: &RColor) {
        unsafe {
            self.set_foreground(&color.inner);
        }
    }
    fn set_background_color(&self, color: &RColor) {
        unsafe {
            self.set_background(&color.inner);
        }
    }
}

impl Colored for QTextEdit {
    fn foreground_color(&self) -> RColor {
        unsafe { RColor::from(self.text_color()) }
    }
    fn background_color(&self) -> RColor {
        self.palette_color(ColorRole::Base)
    }
    fn set_foreground_color(&self, color: &RColor) {
        unsafe {
            self.set_text_color(color.color());
        }
    }
    fn set_background_color(&self, color: &RColor) {
        self.set_palette_color(ColorRole::Base, color);
    }
}

impl Colored for QWidget {
    fn foreground_color(&self) -> RColor {
        self.palette_color(unsafe { self.foreground_role() })
    }
    fn background_color(&self) -> RColor {
        self.palette_color(unsafe { self.background_role() })
    }
    fn set_foreground_color(&self, color: &RColor) {
        self.set_palette_color(unsafe { self.foreground_role() }, color);
    }
    fn set_background_color(&self, color: &RColor) {
        self.set_palette_color(unsafe { self.background_role() }, color);
    }
    fn set_colors(&self, colors: &RColorPair) {
        unsafe {
            let palette = self.palette();
            palette.set_brush_2a(self.foreground_role(), &colors.foreground.inner);
            palette.set_brush_2a(self.background_role(), &colors.background.inner);
            self.set_palette(palette);
        }
    }
}

impl<'de> Deserialize<'de> for RColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        c_uint::deserialize(deserializer).map(Self::from_code)
    }
}

impl Serialize for RColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.code.serialize(serializer)
    }
}
