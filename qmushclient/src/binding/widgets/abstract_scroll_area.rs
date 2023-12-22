use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QAbstractScrollArea;

use super::frame::FrameBinding;
use crate::binding::text::RScrollBar;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RAbstractScrollArea {
    pub(super) inner: QPtr<QAbstractScrollArea>,
}

impl RAbstractScrollArea {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QAbstractScrollArea>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct AbstractScrollAreaBinding {
    inner: QAbstractScrollArea,
}

impl AbstractScrollAreaBinding {
    pub(super) fn cast(inner: &QAbstractScrollArea) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QAbstractScrollArea as *const Self) }
    }

    pub fn horizontal_scroll_bar(&self) -> RScrollBar {
        RScrollBar::get_horizontal(&self.inner)
    }

    pub fn vertical_scroll_bar(&self) -> RScrollBar {
        RScrollBar::get_horizontal(&self.inner)
    }
}

impl Deref for AbstractScrollAreaBinding {
    type Target = FrameBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for RAbstractScrollArea {
    type Target = AbstractScrollAreaBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
