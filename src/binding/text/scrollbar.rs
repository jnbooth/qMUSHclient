use std::os::raw::c_int;

use qt_core::{Orientation, QPtr};
use qt_widgets::q_abstract_slider::SliderAction;
use qt_widgets::{QAbstractScrollArea, QScrollBar};

pub struct ScrollBar<'a> {
    pub(super) inner: QPtr<QScrollBar>,
    pub(super) _owner: &'a QAbstractScrollArea,
}

impl<'a> ScrollBar<'a> {
    /// Returns a pointer to a `QAbstractScrollArea`'s horizontal [`QScrollBar`].
    pub fn get_horizontal(widget: &'a QAbstractScrollArea) -> Self {
        Self {
            // SAFETY: `_owner` restricts `self` to the lifetime of the parent widget
            inner: unsafe { widget.horizontal_scroll_bar() },
            _owner: widget,
        }
    }
    /// Returns a pointer to a `QAbstractScrollArea`'s horizontal [`QScrollBar`].
    pub fn get_vertical(widget: &'a QAbstractScrollArea) -> Self {
        Self {
            // SAFETY: `_owner` restricts `self` to the lifetime of the parent widget
            inner: unsafe { widget.vertical_scroll_bar() },
            _owner: widget,
        }
    }

    pub fn set_range(&self, min: c_int, max: c_int) {
        unsafe { self.inner.set_range(min, max) }
    }

    pub fn trigger_action(&self, action: SliderAction) {
        unsafe { self.inner.trigger_action(action) }
    }

    pub fn is_displayed(&self) -> bool {
        self.minimum() != self.maximum()
    }

    qt_field!(has_tracking, set_tracking, bool);

    qt_field!(inverted_appearance, set_inverted_appearance, bool);

    qt_field!(is_slider_down, set_slider_down, bool);

    qt_field!(maximum, set_maximum, c_int);

    qt_field!(minimum, set_minimum, c_int);

    qt_field!(orientation, set_orientation, Orientation);

    qt_field!(page_step, set_page_step, c_int);

    qt_field!(single_step, set_single_step, c_int);

    qt_field!(slider_position, set_slider_position, c_int);

    qt_field!(value, set_value, c_int);
}
