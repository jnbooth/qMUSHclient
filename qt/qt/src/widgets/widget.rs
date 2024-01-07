use std::ops::Deref;
use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_core::QPtr;
use qt_gui::QCursor;
use qt_widgets as q;

use crate::color::{Colored, QColor};
use crate::font::QFontMetrics;
use crate::shapes::{QPoint, QRect};
use crate::traits::Widget;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QWidget {
    pub(super) inner: QPtr<q::QWidget>,
}

#[repr(transparent)]
pub struct WidgetBinding {
    inner: q::QWidget,
}

impl WidgetBinding {
    pub(super) fn cast(inner: &q::QWidget) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QWidget as *const WidgetBinding) }
    }

    pub fn font_metrics(&self) -> QFontMetrics {
        QFontMetrics::new(unsafe { self.inner.font_metrics() })
    }

    pub fn height(&self) -> c_int {
        unsafe { self.inner.height() }
    }

    pub fn rect(&self) -> QRect {
        QRect::from(unsafe { &*self.inner.rect() })
    }

    pub fn width(&self) -> c_int {
        unsafe { self.inner.width() }
    }

    /// Finds the coordinates of the mouse relative to the widget.
    pub fn cursor_position(&self) -> QPoint {
        unsafe {
            let point = self.inner.map_from_global(&QCursor::pos_0a());
            QPoint::from(&*point)
        }
    }
}

impl Colored for WidgetBinding {
    fn foreground_color(&self) -> QColor {
        self.inner.foreground_color()
    }
    fn set_foreground_color(&self, color: &QColor) {
        self.inner.set_foreground_color(color);
    }
    fn background_color(&self) -> QColor {
        self.inner.background_color()
    }
    fn set_background_color(&self, color: &QColor) {
        self.inner.set_background_color(color)
    }
}

impl Deref for QWidget {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Widget for QWidget {
    fn widget(&self) -> Ptr<q::QWidget> {
        unsafe { self.inner.as_ptr() }
    }
}
