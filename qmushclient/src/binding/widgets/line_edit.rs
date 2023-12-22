use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QLineEdit;

use super::WidgetBinding;
use crate::binding::Printable;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RLineEdit {
    pub(super) inner: QPtr<QLineEdit>,
}

impl RLineEdit {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QLineEdit>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct LineEditBinding {
    inner: QLineEdit,
}

impl LineEditBinding {
    pub(super) fn cast(inner: &QLineEdit) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QLineEdit as *const Self) }
    }

    pub fn text(&self) -> String {
        unsafe { self.inner.text().to_std_string() }
    }

    pub fn set_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_text(&text.to_print()) }
    }
}

impl Deref for LineEditBinding {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for RLineEdit {
    type Target = LineEditBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
