use cpp_core::Ptr;
use qt_core::QPtr;
use qt_widgets as q;

use crate::traits::Widget;

qt_binding!(TextBrowserBinding, q::QTextBrowser, super::TextEditBinding);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QTextBrowser {
    pub(crate) inner: QPtr<q::QTextBrowser>,
}

impl_deref_binding!(QTextBrowser, TextBrowserBinding);

impl QTextBrowser {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QTextBrowser>) -> Self {
        Self { inner }
    }
}

impl Widget for QTextBrowser {
    fn widget(&self) -> Ptr<q::QWidget> {
        unsafe { self.inner.as_ptr().static_upcast() }
    }
}
