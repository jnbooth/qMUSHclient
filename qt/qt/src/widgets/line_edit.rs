use qt_core::QPtr;
use qt_widgets as q;

use crate::traits::Printable;

qt_binding!(LineEditBinding, q::QLineEdit, super::widget::WidgetBinding);

impl LineEditBinding {
    pub fn text(&self) -> String {
        unsafe { self.inner.text().to_std_string() }
    }

    pub fn set_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_text(&text.to_print()) }
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QLineEdit {
    pub(crate) inner: QPtr<q::QLineEdit>,
}

impl_deref_binding!(QLineEdit, LineEditBinding);

impl QLineEdit {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QLineEdit>) -> Self {
        Self { inner }
    }
}
