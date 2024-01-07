use std::ops::Deref;

use qt_core::QPtr;
use qt_widgets as q;

use super::AbstractScrollAreaBinding;
use crate::text::QTextCursor;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct QTextEdit {
    pub(super) inner: QPtr<q::QTextEdit>,
}

#[repr(transparent)]
pub struct TextEditBinding {
    inner: q::QTextEdit,
}

impl TextEditBinding {
    pub(super) fn cast(inner: &q::QTextEdit) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QTextEdit as *const Self) }
    }

    pub fn text_cursor(&self) -> QTextCursor {
        QTextCursor::get(&self.inner)
    }
}

impl Deref for TextEditBinding {
    type Target = AbstractScrollAreaBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for QTextEdit {
    type Target = TextEditBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
