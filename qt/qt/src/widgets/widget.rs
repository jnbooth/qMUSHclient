use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_core::QBox;
use qt_gui::QCursor;
use qt_widgets as q;

use crate::color::{Colored, QColor};
use crate::font::QFontMetrics;
use crate::object::ObjectBinding;
use crate::shapes::QPoint;
use crate::traits::Widget;
use crate::QRect;

qt_binding!(WidgetBinding, q::QWidget, ObjectBinding);

impl WidgetBinding {
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

#[repr(transparent)]
#[derive(Debug)]
pub struct QWidget {
    pub(crate) inner: QBox<q::QWidget>,
}

impl_deref_binding!(QWidget, WidgetBinding);

impl Widget for QWidget {
    fn widget(&self) -> Ptr<q::QWidget> {
        unsafe { self.inner.as_ptr() }
    }
}
