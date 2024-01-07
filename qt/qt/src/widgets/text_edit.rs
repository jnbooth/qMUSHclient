use qt_core::QBox;
use qt_widgets as q;

use super::AbstractScrollAreaBinding;
use crate::text::QTextCursor;

qt_binding!(TextEditBinding, q::QTextEdit, AbstractScrollAreaBinding);

impl TextEditBinding {
    pub fn text_cursor(&self) -> QTextCursor {
        QTextCursor::get(&self.inner)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct QTextEdit {
    pub(crate) inner: QBox<q::QTextEdit>,
}

impl_deref_binding!(QTextEdit, TextEditBinding);
