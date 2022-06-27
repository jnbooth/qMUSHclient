use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QFrame;

use crate::binding::widgets::RWidget;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RFrame {
    pub(super) inner: QPtr<QFrame>,
}

impl Deref for RFrame {
    type Target = RWidget;

    fn deref(&self) -> &Self::Target {
        // SAFETY: repr(transparent)
        unsafe { &*(self.inner.deref() as *const QFrame as *const RWidget) }
    }
}

impl RFrame {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QFrame>) -> Self {
        Self { inner }
    }
}
