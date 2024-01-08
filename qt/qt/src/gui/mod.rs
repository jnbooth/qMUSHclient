use std::os::raw::c_int;

use cpp_core::StaticUpcast;
use qt_core::{QObject, QPtr};
pub use qt_gui::q_font::StyleHint;
pub use qt_gui::q_font_database::SystemFont;
pub use qt_gui::q_palette::ColorRole;
pub use qt_gui::q_text_cursor::{MoveOperation, SelectionType};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;

mod gui_application;
pub(crate) use gui_application::GuiApplicationBinding;
pub use gui_application::QGuiApplication;

mod color;
pub use color::{QColor, QColorPair};

mod cursor;
pub use cursor::{Formats, QTextCursor};

mod document;
pub use document::{
    QTextBlock, QTextDocument, QTextFragment, QTextFrame, QTextLayout, QTextList, QTextTable,
};

mod font;
pub use font::{QFont, QFontMetrics};

mod format;
pub use format::{
    QTextBlockFormat, QTextCharFormat, QTextFormat, QTextFrameFormat, QTextImageFormat,
    QTextListFormat, QTextTableFormat,
};

mod graphics;
pub use graphics::{QImage, QPainter};

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
