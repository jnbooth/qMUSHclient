use cpp_core::{CastFrom, Ptr};
use qt_core::QPtr;
use qt_widgets as q;

use crate::refs::QRef;
use crate::traits::{Printable, Widget, WidgetParent};

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
    pub(crate) inner: QRef<q::QLineEdit>,
}

impl_deref_binding!(QLineEdit, LineEditBinding);

impl Widget for QLineEdit {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}

impl QLineEdit {
    pub fn new<P: WidgetParent>(parent: P) -> Self {
        Self {
            // SAFETY: parent.as_parent() is valid and non-null
            inner: unsafe { q::QLineEdit::from_q_widget(parent.as_parent()).into() },
        }
    }

    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn wrap(inner: QPtr<q::QLineEdit>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}
