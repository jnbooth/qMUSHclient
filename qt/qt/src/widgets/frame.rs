use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets as q;

use super::WidgetBinding;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QFrame {
    pub(super) inner: QPtr<q::QFrame>,
}

impl QFrame {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QFrame>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct FrameBinding {
    inner: q::QFrame,
}

impl FrameBinding {
    pub(super) fn cast(inner: &q::QFrame) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QFrame as *const Self) }
    }
}

impl Deref for FrameBinding {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for QFrame {
    type Target = FrameBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
