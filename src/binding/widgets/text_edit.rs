use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets::QTextEdit;

use crate::binding::text::RTextCursor;
use crate::binding::widgets::abstract_scroll_area::AbstractScrollAreaBinding;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RTextEdit {
    pub(super) inner: QPtr<QTextEdit>,
}

impl RTextEdit {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QTextEdit>) -> Self {
        Self { inner }
    }
}

#[repr(transparent)]
pub struct TextEditBinding {
    inner: QTextEdit,
}

impl TextEditBinding {
    pub(super) fn cast(inner: &QTextEdit) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QTextEdit as *const Self) }
    }

    pub fn text_cursor(&self) -> RTextCursor {
        RTextCursor::get(&self.inner)
    }
}

impl Deref for TextEditBinding {
    type Target = AbstractScrollAreaBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for RTextEdit {
    type Target = TextEditBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
