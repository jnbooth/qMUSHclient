use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QAbstractScrollArea;

use crate::binding::text::RScrollBar;
use crate::binding::widgets::RFrame;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RAbstractScrollArea {
    pub(super) inner: QPtr<QAbstractScrollArea>,
}

impl Deref for RAbstractScrollArea {
    type Target = RFrame;

    fn deref(&self) -> &Self::Target {
        // SAFETY: repr(transparent)
        unsafe { &*(self.inner.deref() as *const QAbstractScrollArea as *const RFrame) }
    }
}

impl RAbstractScrollArea {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QAbstractScrollArea>) -> Self {
        Self { inner }
    }

    pub fn horizontal_scroll_bar(&self) -> RScrollBar {
        RScrollBar::get_horizontal(&self.inner)
    }

    pub fn vertical_scroll_bar(&self) -> RScrollBar {
        RScrollBar::get_horizontal(&self.inner)
    }
}
