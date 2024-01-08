use std::os::raw::c_int;

use qt_core::{Orientation, QPtr};
use qt_widgets as q;
use qt_widgets::q_abstract_slider::SliderAction;

pub struct QScrollBar<'a> {
    pub(crate) inner: QPtr<q::QScrollBar>,
    pub(crate) _owner: &'a q::QAbstractScrollArea,
}

impl<'a> QScrollBar<'a> {
    /// Returns a pointer to a `QAbstractScrollArea`'s horizontal [`QScrollBar`].
    pub fn get_horizontal(widget: &'a q::QAbstractScrollArea) -> Self {
        Self {
            // SAFETY: `_owner` restricts `self` to the lifetime of the parent widget
            inner: unsafe { widget.horizontal_scroll_bar() },
            _owner: widget,
        }
    }
    /// Returns a pointer to a `QAbstractScrollArea`'s horizontal [`QScrollBar`].
    pub fn get_vertical(widget: &'a q::QAbstractScrollArea) -> Self {
        Self {
            // SAFETY: `_owner` restricts `self` to the lifetime of the parent widget
            inner: unsafe { widget.vertical_scroll_bar() },
            _owner: widget,
        }
    }

    pub fn has_tracking(&self) -> bool {
        unsafe { self.inner.has_tracking() }
    }
    pub fn set_tracking(&self, has_tracking: bool) {
        unsafe { self.inner.set_tracking(has_tracking) }
    }

    pub fn maximum(&self) -> c_int {
        unsafe { self.inner.maximum() }
    }
    pub fn set_maximum(&self, maximum: c_int) {
        unsafe { self.inner.set_maximum(maximum) }
    }

    pub fn minimum(&self) -> c_int {
        unsafe { self.inner.minimum() }
    }
    pub fn set_minimum(&self, minimum: c_int) {
        unsafe { self.inner.set_minimum(minimum) }
    }

    pub fn inverted_appearance(&self) -> bool {
        unsafe { self.inner.inverted_appearance() }
    }
    pub fn set_inverted_appearance(&self, inverted_appearance: bool) {
        unsafe { self.inner.set_inverted_appearance(inverted_appearance) }
    }

    pub fn is_slider_down(&self) -> bool {
        unsafe { self.inner.is_slider_down() }
    }
    pub fn set_slider_down(&self, is_slider_down: bool) {
        unsafe { self.inner.set_slider_down(is_slider_down) }
    }

    pub fn is_displayed(&self) -> bool {
        self.minimum() != self.maximum()
    }

    pub fn orientation(&self) -> Orientation {
        unsafe { self.inner.orientation() }
    }
    pub fn set_orientation(&self, orientation: Orientation) {
        unsafe { self.inner.set_orientation(orientation) }
    }

    pub fn set_range(&self, min: c_int, max: c_int) {
        unsafe { self.inner.set_range(min, max) }
    }

    pub fn page_step(&self) -> c_int {
        unsafe { self.inner.page_step() }
    }
    pub fn set_page_step(&self, page_step: c_int) {
        unsafe { self.inner.set_page_step(page_step) }
    }

    pub fn single_step(&self) -> c_int {
        unsafe { self.inner.single_step() }
    }
    pub fn set_single_step(&self, single_step: c_int) {
        unsafe { self.inner.set_single_step(single_step) }
    }

    pub fn slider_position(&self) -> c_int {
        unsafe { self.inner.slider_position() }
    }
    pub fn set_slider_position(&self, slider_position: c_int) {
        unsafe { self.inner.set_slider_position(slider_position) }
    }

    pub fn trigger_action(&self, action: SliderAction) {
        unsafe { self.inner.trigger_action(action) }
    }

    pub fn value(&self) -> c_int {
        unsafe { self.inner.value() }
    }
    pub fn set_value(&self, value: c_int) {
        unsafe { self.inner.set_value(value) }
    }
}
