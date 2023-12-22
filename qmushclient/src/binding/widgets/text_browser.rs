use std::ops::Deref;

use cpp_core::Ptr;
use qt_core::QPtr;
use qt_widgets::{QTextBrowser, QWidget};

use super::TextEditBinding;
use crate::binding::traits::Widget;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RTextBrowser {
    pub(super) inner: QPtr<QTextBrowser>,
}

impl RTextBrowser {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QTextBrowser>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct TextBrowserBinding {
    inner: QTextBrowser,
}

impl TextBrowserBinding {
    pub(super) fn cast(inner: &QTextBrowser) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QTextBrowser as *const Self) }
    }
}

impl Deref for TextBrowserBinding {
    type Target = TextEditBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for RTextBrowser {
    type Target = TextBrowserBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Widget for RTextBrowser {
    fn widget(&self) -> Ptr<QWidget> {
        unsafe { self.inner.as_ptr().static_upcast() }
    }
}
