use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets as q;

use super::frame::FrameBinding;
use crate::text::QScrollBar;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QAbstractScrollArea {
    pub(super) inner: QPtr<q::QAbstractScrollArea>,
}

impl QAbstractScrollArea {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QAbstractScrollArea>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct AbstractScrollAreaBinding {
    inner: q::QAbstractScrollArea,
}

impl AbstractScrollAreaBinding {
    pub(super) fn cast(inner: &q::QAbstractScrollArea) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QAbstractScrollArea as *const Self) }
    }

    pub fn horizontal_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_horizontal(&self.inner)
    }

    pub fn vertical_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_horizontal(&self.inner)
    }
}

impl Deref for AbstractScrollAreaBinding {
    type Target = FrameBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for QAbstractScrollArea {
    type Target = AbstractScrollAreaBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
