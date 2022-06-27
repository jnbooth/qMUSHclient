use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QLineEdit;

use crate::binding::widgets::RWidget;
use crate::binding::Printable;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RLineEdit {
    pub(super) inner: QPtr<QLineEdit>,
}

impl Deref for RLineEdit {
    type Target = RWidget;

    fn deref(&self) -> &Self::Target {
        // SAFETY: repr(transparent)
        unsafe { &*(self.inner.deref() as *const QLineEdit as *const RWidget) }
    }
}

impl RLineEdit {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QLineEdit>) -> Self {
        Self { inner }
    }

    pub fn text(&self) -> String {
        unsafe { self.inner.text().to_std_string() }
    }

    pub fn set_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_text(&text.to_print()) }
    }
}
