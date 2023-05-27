use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QFrame;

use crate::binding::widgets::widget::WidgetBinding;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RFrame {
    pub(super) inner: QPtr<QFrame>,
}

impl RFrame {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QFrame>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct FrameBinding {
    inner: QFrame,
}

impl FrameBinding {
    pub(super) fn cast(inner: &QFrame) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QFrame as *const Self) }
    }
}

impl Deref for FrameBinding {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for RFrame {
    type Target = FrameBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
