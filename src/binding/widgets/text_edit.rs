use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QTextEdit;

use super::RAbstractScrollArea;
use crate::binding::color::Colored;
use crate::binding::text::RTextCursor;
use crate::binding::RColor;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RTextEdit {
    pub(super) inner: QPtr<QTextEdit>,
}

impl Deref for RTextEdit {
    type Target = RAbstractScrollArea;

    fn deref(&self) -> &Self::Target {
        // SAFETY: repr(transparent)
        unsafe { &*(self.inner.deref() as *const QTextEdit as *const RAbstractScrollArea) }
    }
}

impl Colored for RTextEdit {
    fn foreground_color(&self) -> RColor {
        self.inner.foreground_color()
    }
    fn set_foreground_color(&self, color: &RColor) {
        self.inner.set_foreground_color(color);
    }
    fn background_color(&self) -> RColor {
        self.inner.background_color()
    }
    fn set_background_color(&self, color: &RColor) {
        self.inner.set_background_color(color)
    }
}

impl RTextEdit {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QTextEdit>) -> Self {
        Self { inner }
    }

    pub fn text_cursor(&self) -> RTextCursor {
        // SAFETY: `inner` is valid
        unsafe { RTextCursor::get(&self.inner) }
    }
}
