use cpp_core::{CastFrom, Ptr};
use qt_core::QPtr;
use qt_widgets as q;

use crate::traits::Widget;

qt_binding!(
    TextBrowserBinding,
    q::QTextBrowser,
    super::text_edit::TextEditBinding
);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QTextBrowser {
    pub(crate) inner: QPtr<q::QTextBrowser>,
}

impl_deref_binding!(QTextBrowser, TextBrowserBinding);

impl Widget for QTextBrowser {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}

impl QTextBrowser {
    /// # Safety
    ///
    /// Inner must be valid and non-null.
    pub unsafe fn wrap(inner: QPtr<q::QTextBrowser>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}
