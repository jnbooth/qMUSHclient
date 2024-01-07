use std::ops::Deref;

use cpp_core::Ptr;
use qt_core::QPtr;
use qt_widgets as q;

use super::TextEditBinding;
use crate::traits::Widget;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QTextBrowser {
    pub(super) inner: QPtr<q::QTextBrowser>,
}

impl QTextBrowser {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QTextBrowser>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct TextBrowserBinding {
    inner: q::QTextBrowser,
}

impl TextBrowserBinding {
    pub(super) fn cast(inner: &q::QTextBrowser) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QTextBrowser as *const Self) }
    }
}

impl Deref for TextBrowserBinding {
    type Target = TextEditBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for QTextBrowser {
    type Target = TextBrowserBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Widget for QTextBrowser {
    fn widget(&self) -> Ptr<q::QWidget> {
        unsafe { self.inner.as_ptr().static_upcast() }
    }
}
