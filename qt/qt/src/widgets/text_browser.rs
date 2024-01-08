use cpp_core::{CastFrom, Ptr};
use qt_core::QPtr;
use qt_widgets as q;

use crate::refs::QRef;
use crate::traits::{Widget, WidgetParent};

qt_binding!(
    TextBrowserBinding,
    q::QTextBrowser,
    super::text_edit::TextEditBinding
);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QTextBrowser {
    pub(crate) inner: QRef<q::QTextBrowser>,
}

impl_deref_binding!(QTextBrowser, TextBrowserBinding);

impl Widget for QTextBrowser {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}

impl QTextBrowser {
    pub fn new<P: WidgetParent>(parent: P) -> Self {
        Self {
            // SAFETY: parent.as_parent() is valid and non-null
            inner: unsafe { q::QTextBrowser::new_1a(parent.as_parent()).into() },
        }
    }

    /// # Safety
    ///
    /// Inner must be valid and non-null.
    pub unsafe fn wrap(inner: QPtr<q::QTextBrowser>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}
