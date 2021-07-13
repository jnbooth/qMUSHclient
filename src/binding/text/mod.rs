use std::os::raw::c_int;

use cpp_core::StaticUpcast;
use qt_core::{QObject, QPtr};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;

mod cursor;
pub use cursor::{Cursor, Formats};
mod document;
pub use document::{Block, Document, Fragment, Frame, Layout, List, Table};
mod format;
pub use format::{
    BlockFormat, CharFormat, FrameFormat, ImageFormat, ListFormat, TableFormat, TextFormat,
};

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
