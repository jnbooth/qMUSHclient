use std::ops::Deref;
use std::os::raw::{c_double, c_int};

use cpp_core::CppBox;
use qt_core::{AlignmentFlag, LayoutDirection, QFlags, QString, QStringList};
use qt_gui as q;
use qt_gui::q_font::Weight;
use qt_gui::q_text_block_format::LineHeightTypes;
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;

use crate::color::{Colored, QColor};
use crate::{Printable, QFont, QList};

fn optional_string(s: CppBox<QString>) -> Option<String> {
    if unsafe { s.is_empty() } {
        None
    } else {
        Some(s.to_std_string())
    }
}

#[repr(transparent)]
pub struct QTextFormat {
    pub(crate) inner: q::QTextFormat,
}

impl QTextFormat {
    fn new(fmt: &q::QTextFormat) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(fmt as *const q::QTextFormat as *const Self) }
    }

    /// Clears the brush used to paint the document's foreground. The default brush will be used.
    pub fn clear_foreground(&self) {
        unsafe {
            self.inner.clear_foreground();
        }
    }

    /// Clears the brush used to paint the document's background. The default brush will be used.
    pub fn clear_background(&self) {
        unsafe {
            self.inner.clear_background();
        }
    }

    qt_field!(layout_direction, set_layout_direction, LayoutDirection);
}

impl Colored for QTextFormat {
    fn foreground_color(&self) -> QColor {
        self.inner.foreground_color()
    }

    fn set_foreground_color(&self, color: &QColor) {
        self.inner.set_foreground_color(color)
    }

    fn background_color(&self) -> QColor {
        self.inner.background_color()
    }

    fn set_background_color(&self, color: &QColor) {
        self.inner.set_background_color(color)
    }
}

macro_rules! impl_fmt {
    ($t:ty, $from:ty) => {
        impl $t {
            pub fn new() -> Self {
                Self {
                    inner: unsafe { <$from>::new() },
                }
            }
        }

        impl Clone for $t {
            fn clone(&self) -> Self {
                Self {
                    inner: unsafe { <$from>::new_copy(&self.inner) },
                }
            }
        }

        impl PartialEq<$t> for $t {
            fn eq(&self, other: &$t) -> bool {
                self.inner.eq(unsafe { &other.inner.static_upcast() })
            }
        }

        impl Eq for $t {}

        impl Default for $t {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Deref for $t {
            type Target = QTextFormat;

            fn deref(&self) -> &Self::Target {
                QTextFormat::new(self.inner.deref())
            }
        }

        impl From<CppBox<$from>> for $t {
            fn from(value: CppBox<$from>) -> Self {
                Self { inner: value }
            }
        }
    };
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextBlockFormat {
    pub(crate) inner: CppBox<q::QTextBlockFormat>,
}
impl_fmt!(QTextBlockFormat, q::QTextBlockFormat);

impl QTextBlockFormat {
    pub fn alignment(&self) -> QFlags<AlignmentFlag> {
        unsafe { self.inner.alignment() }
    }

    pub fn set_alignment<T: Into<QFlags<AlignmentFlag>>>(&self, align: T) {
        unsafe {
            self.inner.set_alignment(align.into());
        }
    }

    qt_field!(heading_level, set_heading_level, c_int);

    pub fn line_height(&self) -> c_double {
        unsafe { self.inner.line_height_0a() / 100.0 }
    }

    pub fn set_line_height(&self, line_height: c_double) {
        unsafe {
            self.inner.set_line_height(
                line_height * 100.0,
                LineHeightTypes::ProportionalHeight.to_int(),
            );
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextCharFormat {
    pub(crate) inner: CppBox<q::QTextCharFormat>,
}
impl_fmt!(QTextCharFormat, q::QTextCharFormat);

impl QTextCharFormat {
    pub fn set_font(&self, font: &QFont) {
        unsafe {
            self.inner.set_font_1a(font);
        }
    }
    pub fn set_bold(&self, enable: bool) {
        unsafe {
            self.inner
                .set_font_weight(if enable { Weight::Bold } else { Weight::Normal }.to_int());
        }
    }
    pub fn set_italic(&self, enable: bool) {
        unsafe {
            self.inner.set_font_italic(enable);
        }
    }
    pub fn set_strikeout(&self, enable: bool) {
        unsafe {
            self.inner.set_font_strike_out(enable);
        }
    }
    pub fn set_underline(&self, enable: bool) {
        unsafe {
            self.inner.set_font_underline(enable);
        }
    }

    pub fn size(&self) -> c_double {
        unsafe { self.inner.font_point_size() }
    }

    pub fn set_size(&self, size: c_double) {
        unsafe {
            self.inner.set_font_point_size(size);
        }
    }

    pub fn is_anchor(&self) -> bool {
        unsafe { self.inner.is_anchor() }
    }

    pub fn set_anchor(&self, enable: bool) {
        unsafe {
            self.inner.set_anchor(enable);
        }
    }

    pub fn anchor_href(&self) -> Option<String> {
        optional_string(unsafe { self.inner.anchor_href() })
    }

    pub fn set_anchor_href(&self, href: &str) {
        unsafe {
            self.inner.set_anchor_href(&QString::from_std_str(href));
        }
    }

    pub fn clear_anchor_href(&self) {
        unsafe {
            self.inner.set_anchor_href(&QString::new());
        }
    }

    pub fn anchor_names(&self) -> Vec<String> {
        unsafe {
            self.inner
                .anchor_names()
                .iter()
                .map(|x| x.to_std_string())
                .collect()
        }
    }

    pub fn set_anchor_names<T: AsRef<str>>(&self, names: &[T]) {
        unsafe {
            let list = QStringList::from_iter(names.iter().map(QString::from_std_str));
            self.inner.set_anchor_names(&list);
        }
    }

    pub fn clear_anchor_names(&self) {
        unsafe {
            self.inner.set_anchor_names(&QStringList::new());
        }
    }

    pub fn tooltip(&self) -> Option<String> {
        optional_string(unsafe { self.inner.tool_tip() })
    }

    pub fn set_tooltip<S: Printable>(&self, tooltip: S) {
        unsafe {
            self.inner.set_tool_tip(&tooltip.to_print());
        }
    }

    pub fn clear_tooltip(&self) {
        unsafe {
            self.inner.set_tool_tip(&QString::new()); // TODO does this actually work?
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextListFormat {
    pub(crate) inner: CppBox<q::QTextListFormat>,
}
impl_fmt!(QTextListFormat, q::QTextListFormat);

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextFrameFormat {
    pub(crate) inner: CppBox<q::QTextFrameFormat>,
}
impl_fmt!(QTextFrameFormat, q::QTextFrameFormat);

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextTableFormat {
    pub(crate) inner: CppBox<q::QTextTableFormat>,
}
impl_fmt!(QTextTableFormat, q::QTextTableFormat);

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextImageFormat {
    pub(crate) inner: CppBox<q::QTextImageFormat>,
}
impl_fmt!(QTextImageFormat, q::QTextImageFormat);
