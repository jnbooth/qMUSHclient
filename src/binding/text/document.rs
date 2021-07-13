use std::os::raw::c_int;

use cpp_core::{CppBox, Ptr};
use qt_core::{LayoutDirection, QPtr};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;
use qt_gui::*;

use super::format::{BlockFormat, CharFormat};
use super::{nonnull, Position};

#[derive(Debug)]
pub struct Block(pub(super) CppBox<QTextBlock>);

impl From<CppBox<QTextBlock>> for Block {
    fn from(value: CppBox<QTextBlock>) -> Self {
        Self(value)
    }
}

impl Block {
    /// Returns the BlockFormat that describes block-specific properties.
    pub fn block_format(&self) -> BlockFormat {
        unsafe { self.0.block_format() }.into()
    }
    /// Returns the number of this block, or -1 if the block is invalid.
    pub fn number(&self) -> c_int {
        unsafe { self.0.block_number() }
    }
    /// Returns the CharFormat that describes the block's character format. The block's character format is used when inserting text into an empty block.
    pub fn char_format(&self) -> CharFormat {
        unsafe { self.0.char_format() }.into()
    }
    /// Clears the Layout that is used to lay out and display the block's contents.
    pub fn clear_layout(&self) {
        unsafe { self.0.clear_layout() }
    }
    /// Returns true if the given position is located within the text block; otherwise returns false.
    pub fn contains(&self, position: Position) -> bool {
        unsafe { self.0.contains(position) }
    }
    /// Returns the text document this text block belongs to, or None if the text block does not belong to any document.
    pub fn document(&self) -> Option<Document> {
        unsafe { nonnull(self.0.document()) }.map(Document)
    }
    /// Returns the first line number of this block, or -1 if the block is invalid. Unless the layout supports it, the line number is identical to the block number.
    pub fn first_line_number(&self) -> c_int {
        unsafe { self.0.first_line_number() }
    }
    /// Returns true if this text block is valid; otherwise returns false.
    pub fn is_valid(&self) -> bool {
        unsafe { self.0.is_valid() }
    }
    // Returns true if the block is visible; otherwise returns false.
    pub fn is_visible(&self) -> bool {
        unsafe { self.0.is_visible() }
    }
    /// Returns the Layout that is used to lay out and display the block's contents.
    pub fn layout(&self) -> Layout {
        Layout(unsafe { self.0.layout() })
    }
    /// Returns the length of the block in characters.
    /// Note: The length returned includes all formatting characters, for example, newline.
    pub fn len(&self) -> c_int {
        unsafe { self.0.length() }
    }

    /// Returns true if the block has no contents.
    pub fn is_empty(&self) -> bool {
        unsafe { self.0.length() == 0 }
    }
    /// Returns the line count. Not all document layouts support this feature.
    pub fn line_count(&self) -> c_int {
        unsafe { self.0.line_count() }
    }
    /// Returns the index of the block's first character within the document.
    pub fn position(&self) -> Position {
        unsafe { self.0.position() }
    }
    /// Returns the block's revision.
    pub fn revision(&self) -> c_int {
        unsafe { self.0.revision() }
    }
    /// Sets a block's revision.
    pub fn set_revision(&self, rev: c_int) {
        unsafe { self.0.set_revision(rev) }
    }
    /// Stores the specified state integer value in the text block. This may be useful for example in a syntax highlighter to store a text parsing state.
    pub fn set_user_state(&self, state: c_int) {
        unsafe { self.0.set_user_state(state) }
    }
    /// Sets the block's visibility.
    pub fn set_visible(&self, visible: bool) {
        unsafe { self.0.set_visible(visible) }
    }
    /// Returns the block's contents as plain text.
    pub fn text(&self) -> String {
        unsafe { self.0.text() }.to_std_string()
    }
    /// Returns the resolved text direction.
    // If the block has no explicit direction set, it will resolve the direction from the blocks content.
    pub fn text_direction(&self) -> LayoutDirection {
        unsafe { self.0.text_direction() }
    }
    /// If the block represents a list item, returns the list that the item belongs to; otherwise returns None.
    pub fn text_list(&self) -> Option<List> {
        unsafe { nonnull(self.0.text_list()) }.map(List)
    }
    /// Returns the integer value previously set with setUserState() or -1.
    pub fn user_state(&self) -> c_int {
        unsafe { self.0.user_state() }
    }
}

#[derive(Debug)]
pub struct Layout(pub(super) Ptr<QTextLayout>);

#[derive(Debug)]
pub struct List(pub(super) QPtr<QTextList>);

#[derive(Debug)]
pub struct Frame(pub(super) QPtr<QTextFrame>);

#[derive(Debug)]
pub struct Table(pub(super) QPtr<QTextTable>);

#[derive(Debug)]
pub struct Fragment(pub(super) CppBox<QTextDocumentFragment>);

#[derive(Debug)]
pub struct Document(pub(super) QPtr<QTextDocument>);
