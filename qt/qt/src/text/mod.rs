use std::os::raw::c_int;

use cpp_core::StaticUpcast;
use qt_core::{QObject, QPtr};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;

mod cursor;
pub use cursor::{Formats, QTextCursor};
mod document;
pub use document::{
    QTextBlock, QTextDocument, QTextFragment, QTextFrame, QTextLayout, QTextList, QTextTable,
};
mod format;
pub use format::{
    QTextBlockFormat, QTextCharFormat, QTextFormat, QTextFrameFormat, QTextImageFormat,
    QTextListFormat, QTextTableFormat,
};
mod scrollbar;
pub use scrollbar::QScrollBar;

pub type Position = c_int;

/// # Safety
///
/// `ptr` must be valid.
unsafe fn nonnull<Q: StaticUpcast<QObject>>(ptr: QPtr<Q>) -> Option<QPtr<Q>> {
    if unsafe { ptr.is_null() } {
        None
    } else {
        Some(ptr)
    }
}

fn if_valid(n: c_int) -> Option<c_int> {
    if n == -1 {
        None
    } else {
        Some(n)
    }
}
