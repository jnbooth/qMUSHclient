use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QTextBrowser;

use super::RTextEdit;
use crate::binding::text::RTextCursor;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RTextBrowser {
    pub(super) inner: QPtr<QTextBrowser>,
}

impl Deref for RTextBrowser {
    type Target = RTextEdit;

    fn deref(&self) -> &Self::Target {
        // SAFETY: repr(transparent)
        unsafe { &*(self.inner.deref() as *const QTextBrowser as *const RTextEdit) }
    }
}

impl RTextBrowser {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QTextBrowser>) -> Self {
        Self { inner }
    }

    pub fn text_cursor(&self) -> RTextCursor {
        // SAFETY: `inner` is valid
        unsafe { RTextCursor::get(&self.inner) }
    }
}
