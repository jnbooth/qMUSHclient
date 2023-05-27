use std::ops::Deref;
use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_core::QPtr;
use qt_gui::QCursor;
use qt_widgets::QWidget;

use super::super::traits::Widget;
use crate::binding::color::{Colored, RColor};
use crate::binding::font::RFontMetrics;
use crate::binding::graphics::RPoint;
use crate::binding::RRect;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RWidget {
    pub(super) inner: QPtr<QWidget>,
}

impl RWidget {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QWidget>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct WidgetBinding {
    inner: QWidget,
}

impl WidgetBinding {
    pub(super) fn cast(inner: &QWidget) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QWidget as *const WidgetBinding) }
    }

    pub fn font_metrics(&self) -> RFontMetrics {
        RFontMetrics::new(unsafe { self.inner.font_metrics() })
    }

    pub fn height(&self) -> c_int {
        unsafe { self.inner.height() }
    }

    pub fn rect(&self) -> RRect {
        RRect::from(unsafe { &*self.inner.rect() })
    }

    pub fn width(&self) -> c_int {
        unsafe { self.inner.width() }
    }

    /// Finds the coordinates of the mouse relative to the widget.
    pub fn cursor_position(&self) -> RPoint {
        unsafe {
            let point = self.inner.map_from_global(&QCursor::pos_0a());
            RPoint::from(&*point)
        }
    }
}

impl Colored for WidgetBinding {
    fn foreground_color(&self) -> RColor {
        self.inner.foreground_color()
    }
    fn set_foreground_color(&self, color: &RColor) {
        self.inner.set_foreground_color(color);
    }
    fn background_color(&self) -> RColor {
        self.inner.background_color()
    }
    fn set_background_color(&self, color: &RColor) {
        self.inner.set_background_color(color)
    }
}

impl Deref for RWidget {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Widget for RWidget {
    fn widget(&self) -> Ptr<QWidget> {
        unsafe { self.inner.as_ptr() }
    }
}
