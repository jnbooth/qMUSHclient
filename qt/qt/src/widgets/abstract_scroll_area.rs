use qt_core::QPtr;
use qt_widgets as q;

use super::scrollbar::QScrollBar;

qt_binding!(
    AbstractScrollAreaBinding,
    q::QAbstractScrollArea,
    super::frame::FrameBinding
);

impl AbstractScrollAreaBinding {
    pub fn horizontal_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_horizontal(&self.inner)
    }

    pub fn vertical_scroll_bar(&self) -> QScrollBar {
        QScrollBar::get_horizontal(&self.inner)
    }
}

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QAbstractScrollArea {
    pub(crate) inner: QPtr<q::QAbstractScrollArea>,
}

impl_deref_binding!(QAbstractScrollArea, AbstractScrollAreaBinding);

impl QAbstractScrollArea {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<q::QAbstractScrollArea>) -> Self {
        Self { inner }
    }
}
