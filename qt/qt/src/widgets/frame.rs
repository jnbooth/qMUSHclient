use qt_core::QPtr;
use qt_widgets as q;

use super::WidgetBinding;

qt_binding!(FrameBinding, q::QFrame, WidgetBinding);

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QFrame {
    pub(crate) inner: QPtr<q::QFrame>,
}

impl_deref_binding!(QFrame, FrameBinding);

impl QFrame {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QFrame>) -> Self {
        Self { inner }
    }
}
