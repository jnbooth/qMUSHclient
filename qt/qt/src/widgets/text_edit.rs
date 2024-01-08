use cpp_core::{CastFrom, Ptr};
use qt_core::QBox;
use qt_widgets as q;

use crate::gui::QTextCursor;
use crate::traits::{Widget, WidgetParent};

qt_binding!(
    TextEditBinding,
    q::QTextEdit,
    super::abstract_scroll_area::AbstractScrollAreaBinding
);

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

impl Widget for QTextEdit {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}

impl QTextEdit {
    pub fn new<P: WidgetParent>(parent: P) -> Self {
        Self {
            // SAFETY: parent.as_parent() is valid
            inner: unsafe { q::QTextEdit::from_q_widget(parent.as_parent()) },
        }
    }
}
