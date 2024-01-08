use cpp_core::{CastFrom, Ptr};
use qt_core::QPtr;
use qt_widgets as q;

use crate::refs::QRef;
use crate::traits::{Widget, WidgetParent};

qt_binding!(FrameBinding, q::QFrame, super::widget::WidgetBinding);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QFrame {
    pub(crate) inner: QRef<q::QFrame>,
}

impl_deref_binding!(QFrame, FrameBinding);

impl Widget for QFrame {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}

impl QFrame {
    pub fn new<P: WidgetParent>(parent: P) -> Self {
        Self {
            // SAFETY: parent.as_parent() is valid
            inner: unsafe { q::QFrame::new_1a(parent.as_parent()).into() },
        }
    }

    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn wrap(inner: QPtr<q::QFrame>) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}
