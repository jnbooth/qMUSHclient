use std::borrow::Borrow;
use std::os::raw::c_int;

use cpp_core::{CastInto, CppBox, Ptr};
use qt_gui::q_text_cursor::{MoveMode, MoveOperation, SelectionType};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;
use qt_gui::*;
use qt_widgets::QTextEdit;

use super::document::{Block, Document, Fragment, Frame, List, Selection, Table};
use super::format::{BlockFormat, CharFormat, FrameFormat, ImageFormat, ListFormat, TableFormat};
use super::{if_valid, nonnull, Position};
use crate::binding::color::{Colored, RColor};
use crate::binding::graphics::RImage;
use crate::binding::printable::Printable;

#[derive(Debug)]
pub struct Formats {
    pub block: BlockFormat,
    pub text: CharFormat,
}

#[derive(Debug)]
pub struct Cursor {
    pub(super) inner: CppBox<QTextCursor>,
    pub format: Formats,
}

impl From<CppBox<QTextCursor>> for Cursor {
    fn from(inner: CppBox<QTextCursor>) -> Self {
        unsafe {
            let format = Formats {
                block: BlockFormat::from(inner.block_format()),
                text: CharFormat::from(inner.char_format()),
            };
            Self { inner, format }
        }
    }
}

impl Cursor {
    /// Returns a copy of a `QTextEdit`'s [`QTextCursor`` that represents the currently visible
    /// cursor.
    ///
    /// # Safety
    ///
    /// `widget` must be valid and non-null.
    pub unsafe fn get<T: CastInto<Ptr<QTextEdit>>>(widget: T) -> Self {
        unsafe { widget.cast_into().text_cursor().into() }
    }
    /// Sets the visible cursor.
    ///
    /// # Safety
    ///
    /// `widget` must be valid and non-null.
    pub unsafe fn set<T: CastInto<Ptr<QTextEdit>>>(&self, widget: T) {
        unsafe { widget.cast_into().set_text_cursor(&self.inner) }
    }
    /// Returns `true` if the cursor is at the end of a block; otherwise returns `false`.
    pub fn at_block_end(&self) -> bool {
        unsafe { self.inner.at_block_end() }
    }
    /// Returns `true` if the cursor is at the start of a block; otherwise returns `false`.
    pub fn at_block_start(&self) -> bool {
        unsafe { self.inner.at_block_start() }
    }
    /// Returns `true` if the cursor is at the end of the document; otherwise returns `false`.
    pub fn at_end(&self) -> bool {
        unsafe { self.inner.at_end() }
    }
    /// Returns `true` if the cursor is at the start of the document; otherwise returns `false`.
    pub fn at_start(&self) -> bool {
        unsafe { self.inner.at_start() }
    }
    /// Indicates a block of editing operations on the document that should appear as a single
    /// operation from an undo/redo point of view.
    pub fn transaction<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Self) -> R,
    {
        unsafe {
            self.inner.begin_edit_block();
            let res = f(self);
            self.inner.end_edit_block();
            res
        }
    }
    /// Returns the block that contains the cursor.
    pub fn block(&self) -> Block {
        unsafe { self.inner.block() }.into()
    }
    /// Returns the position of the cursor within its containing line.
    ///
    /// Note that this is the column number relative to a wrapped line, not relative to the block
    /// (i.e. the paragraph).
    pub fn column(&self) -> c_int {
        unsafe { self.inner.column_number() }
    }
    /// Creates and returns a new list with the given `format` and makes the current paragraph the
    /// cursor is in the first list item.
    pub fn convert_to_list(&self, format: &ListFormat) -> List {
        List(unsafe { self.inner.create_list_q_text_list_format(&format.0) })
    }
    /// Returns a pointer to the current frame. Returns `None` if the cursor is invalid.
    pub fn current_frame(&self) -> Option<Frame> {
        unsafe { nonnull(self.inner.current_frame()) }.map(Frame)
    }
    /// Returns the current list if the cursor position is inside a block that is part of a list;
    /// otherwise returns `None`.
    pub fn current_list(&self) -> Option<List> {
        unsafe { nonnull(self.inner.current_list()) }.map(List)
    }
    /// Returns a pointer to the current table if the cursor position is inside a block that is part
    /// of a table; otherwise returns `None`.
    pub fn current_table(&self) -> Option<Table> {
        unsafe { nonnull(self.inner.current_table()) }.map(Table)
    }
    /// Deletes the character at the current cursor position.
    pub fn delete_char(&self) {
        unsafe { self.inner.delete_char() }
    }
    /// Deletes the character before the current cursor position.
    pub fn delete_previous_char(&self) {
        unsafe { self.inner.delete_previous_char() }
    }
    /// Returns the document this cursor is associated with.
    pub fn document(&self) -> Document {
        Document(unsafe { self.inner.document() })
    }
    /// Inserts a new empty block at the cursor position with the current [`BlockFormat`].
    pub fn insert_block(&self) {
        unsafe {
            self.inner
                .insert_block_2a(&self.format.block.0, &self.format.text.0)
        }
    }
    /// Inserts a new empty block at the cursor position with the given `format`.
    pub fn insert_block_formatted(&self, format: &BlockFormat) {
        unsafe { self.inner.insert_block_2a(&format.0, &self.format.text.0) }
    }
    /// Inserts the text `fragment` at the current position.
    pub fn insert_fragment(&self, fragment: &Fragment) {
        unsafe { self.inner.insert_fragment(&fragment.0) }
    }
    /// Inserts a frame with the given `format` at the current cursor position, moves the cursor
    /// position inside the frame, and returns the frame.
    pub fn insert_frame(&self, format: &FrameFormat) -> Frame {
        Frame(unsafe { self.inner.insert_frame(&format.0) })
    }
    /// Inserts the `text` at the current position. The text is interpreted as HTML.
    pub fn insert_html<S: Printable>(&self, text: S) {
        unsafe { self.inner.insert_html(&text.to_print()) }
    }
    /// Inserts the image defined by the given `format` at the cursor's current position with the
    /// specified `alignment`.
    pub fn insert_image_formatted(&self, format: &ImageFormat, alignment: FramePosition) {
        unsafe {
            self.inner
                .insert_image_q_text_image_format_position(&format.0, alignment)
        }
    }
    /// Convenience method for inserting the image with the given `name` at the current position.
    ///
    /// This uses [`Document::add_resource`].
    pub fn insert_image_resource<S: Printable>(&self, name: S) {
        unsafe { self.inner.insert_image_q_string(&name.to_print()) }
    }
    /// Convenience function for inserting the given `image` with a `name` at the current position.
    pub fn insert_image_named<S: Printable>(&self, image: &RImage, name: S) {
        unsafe {
            self.inner
                .insert_image_q_image_q_string(&image.0, &name.to_print())
        }
    }
    /// Convenience function for inserting the given `image` at the current position.
    pub fn insert_image(&self, image: &RImage) {
        unsafe { self.inner.insert_image_q_image(&image.0) }
    }
    /// Inserts a new block at the current position and makes it the first list item of a newly
    /// created list with the given `format`. Returns the created list.
    pub fn insert_list(&self, format: &ListFormat) -> List {
        List(unsafe { self.inner.insert_list_q_text_list_format(&format.0) })
    }
    /// Creates a new table with the given `format` and the given number of `rows` and `columns`,
    /// inserts it at the current cursor position in the document, and returns the table object. The
    /// cursor is moved to the beginning of the first cell.
    ///
    /// There must be at least one row and one column in the table.
    pub fn insert_table(&self, rows: c_int, columns: c_int, format: &TableFormat) -> Table {
        Table(unsafe { self.inner.insert_table_3a(rows, columns, &format.0) })
    }
    /// Inserts text at the current position, using the current [`CharFormat`].
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text<S: Printable>(&self, text: S) {
        unsafe {
            self.inner
                .insert_text_2a(&text.to_print(), &self.format.text.0)
        }
    }
    /// Inserts text at the current position, using the given `format`.
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text_formatted<S: Printable>(&self, text: S, format: &CharFormat) {
        unsafe { self.inner.insert_text_2a(&text.to_print(), &format.0) }
    }
    /// Inserts text at the current position, using the current character format plus text color.
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text_colored<S, B>(&self, text: S, fg: Option<B>, bg: Option<B>)
    where
        S: Printable,
        B: Borrow<RColor>,
    {
        unsafe {
            let fmt = QTextCharFormat::new_copy(&self.format.text.0);
            if let Some(fg) = fg {
                fmt.set_foreground_color(fg.borrow());
            }
            if let Some(bg) = bg {
                fmt.set_background_color(bg.borrow());
            }
            self.inner.insert_text_2a(&text.to_print(), &fmt);
        }
    }
    /// Moves the cursor by performing the given operation `n` times, using the specified `mode`,
    /// and returns `true` if all operations were completed successfully; otherwise returns `false`.
    ///
    /// For example, if this function is repeatedly used to seek to the end of the next word, it
    /// will eventually fail when the end of the document is reached.
    pub fn move_position(&self, mode: MoveOperation, n: c_int) -> bool {
        unsafe { self.inner.move_position_3a(mode, MoveMode::MoveAnchor, n) }
    }
    /// Returns the absolute position of the cursor within the document. The cursor is positioned
    /// between characters.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn position(&self) -> Position {
        unsafe { self.inner.position() }
    }
    /// Returns the relative position of the cursor within the block. The cursor is positioned
    /// between characters.
    ///
    /// This is equivalent to [`position()`](Self::position)` -
    /// `[`block()`](Self::block)`.`[`position()`](Block::position).
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn position_in_block(&self) -> Position {
        unsafe { self.inner.position_in_block() }
    }
    /// Selects text in the document according to the given `selection` type.
    pub fn select(&self, selection: SelectionType) -> Selection {
        unsafe {
            let cursor = QTextCursor::new_copy(&self.inner);
            cursor.select(selection);
            Selection(cursor)
        }
    }
    /// Moves the cursor to the absolute position in the document specified by `pos`. The cursor is
    /// positioned between characters.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and pos is considered an index into this string.
    /// This does not necessarily correspond to individual graphemes in the writing system, as a
    /// single grapheme may be represented by multiple Unicode characters, such as in the case of
    /// surrogate pairs, linguistic ligatures or diacritics. For a more generic approach to
    /// navigating the document, use [`move_position`](Self::move_position), which will respect the
    /// actual grapheme boundaries in the text.
    pub fn set_position(&self, pos: Position) {
        unsafe { self.inner.set_position_1a(pos) }
    }
    /// Sets the visual x position for vertical cursor movements to `x`.
    ///
    /// The vertical movement x position is cleared automatically when the cursor moves
    /// horizontally, and kept unchanged when the cursor moves vertically. The mechanism allows the
    /// cursor to move up and down on a visually straight line with proportional fonts, and to
    /// gently "jump" over short lines.
    pub fn set_vertical_movement_x(&self, x: Position) {
        unsafe { self.inner.set_vertical_movement_x(x) }
    }
    /// Unsets the visual x position for vertical cursor movements.
    ///
    /// It will then be set automatically the next time the cursor moves up or down.
    pub fn unset_vertical_movement_x(&self) {
        unsafe { self.inner.set_vertical_movement_x(-1) }
    }
    /// Swaps this text cursor instance with `other`. This function is very fast and never fails.
    pub fn swap(&self, other: &Self) {
        unsafe { self.inner.swap(&other.inner) }
    }
    /// Returns the visual x position for vertical cursor movements.
    ///
    /// A value of `None` indicates no predefined x position. It will then be set automatically the
    /// next time the cursor moves up or down.
    pub fn vertical_movement_x(&self) -> Option<Position> {
        if_valid(unsafe { self.inner.vertical_movement_x() })
    }
}
