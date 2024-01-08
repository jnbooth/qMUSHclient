use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_core::QBox;
use qt_gui::q_palette::ColorRole;
use qt_gui::QCursor;
use qt_widgets as q;

use crate::core::{ObjectBinding, QPoint, QRect};
use crate::gui::{QColor, QFontMetrics};
use crate::traits::{Colored, Printable, Widget};

qt_binding!(WidgetBinding, q::QWidget, ObjectBinding);

impl WidgetBinding {
    pub fn accept_drops(&self) -> bool {
        unsafe { self.inner.accept_drops() }
    }
    pub fn set_accept_drops(&self, accept_drops: bool) {
        unsafe { self.inner.set_accept_drops(accept_drops) }
    }

    /// This property holds the widget's description as seen by assistive technologies.
    pub fn accessible_description(&self) -> String {
        unsafe { self.inner.accessible_description().to_std_string() }
    }
    /// This property holds the widget's description as seen by assistive technologies.
    pub fn set_accessible_description<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_accessible_description(&text.to_print()) }
    }

    /// This property holds the widget's name as seen by assistive technologies.
    pub fn accessible_name(&self) -> String {
        unsafe { self.inner.accessible_name().to_std_string() }
    }
    /// This property holds the widget's name as seen by assistive technologies.
    pub fn set_accessible_name<S: Printable>(&self, name: S) {
        unsafe { self.inner.set_accessible_name(&name.to_print()) }
    }

    pub fn auto_fill_background(&self) -> bool {
        unsafe { self.inner.auto_fill_background() }
    }
    pub fn set_auto_fill_background(&self, auto_fill_background: bool) {
        unsafe { self.inner.set_auto_fill_background(auto_fill_background) }
    }

    pub fn background_role(&self) -> ColorRole {
        unsafe { self.inner.background_role() }
    }
    pub fn set_background_role(&self, role: ColorRole) {
        unsafe { self.inner.set_background_role(role) }
    }

    /// Finds the coordinates of the mouse relative to the widget.
    pub fn cursor_position(&self) -> QPoint {
        unsafe {
            let point = self.inner.map_from_global(&QCursor::pos_0a());
            QPoint::from(&*point)
        }
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
