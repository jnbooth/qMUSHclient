use cpp_core::{NullPtr, Ptr};
use qt_widgets as q;

use super::widget::Widget;

pub trait WidgetParent {
    /// # Safety
    ///
    /// The returned pointer is nullable, but it must be valid.
    #[allow(clippy::wrong_self_convention)]
    fn as_parent(self) -> Ptr<q::QWidget>;
}

impl<T: Widget> WidgetParent for &T {
    fn as_parent(self) -> Ptr<q::QWidget> {
        self.widget()
    }
}

impl WidgetParent for NullPtr {
    fn as_parent(self) -> Ptr<q::QWidget> {
        unsafe { Ptr::null() }
    }
}

impl WidgetParent for () {
    fn as_parent(self) -> Ptr<q::QWidget> {
        unsafe { Ptr::null() }
    }
}

impl WidgetParent for Ptr<q::QWidget> {
    fn as_parent(self) -> Ptr<q::QWidget> {
        self
    }
}
