use std::borrow::Borrow;
use std::os::raw::c_int;

use cpp_core::{CastInto, CppBox, Ptr};
use qt_gui::q_text_cursor::{MoveMode, MoveOperation, SelectionType};
use qt_gui::q_text_frame_format::Position as FramePosition;
use {qt_gui as q, qt_widgets as w};

use super::color::QColor;
use super::document::{
    QTextBlock, QTextDocument, QTextFragment, QTextFrame, QTextList, QTextTable, Selection,
};
use super::format::{
    QTextBlockFormat, QTextCharFormat, QTextFrameFormat, QTextImageFormat, QTextListFormat,
    QTextTableFormat,
};
use super::graphics::QImage;
use super::{if_valid, nonnull, Position};
use crate::traits::{Colored, Printable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Formats {
    pub block: QTextBlockFormat,
    pub text: QTextCharFormat,
}

#[derive(Debug)]
pub struct QTextCursor {
    pub(crate) inner: CppBox<q::QTextCursor>,
    pub format: Formats,
}

impl_eq_cpp!(QTextCursor);

impl Clone for QTextCursor {
    fn clone(&self) -> Self {
        Self {
            inner: unsafe { q::QTextCursor::new_copy(&self.inner) },
            format: self.format.clone(),
        }
    }
}

impl From<CppBox<q::QTextCursor>> for QTextCursor {
    fn from(inner: CppBox<q::QTextCursor>) -> Self {
        unsafe {
            let format = Formats {
                block: QTextBlockFormat::from(inner.block_format()),
                text: QTextCharFormat::from(inner.char_format()),
            };
            Self { inner, format }
        }
    }
}

impl QTextCursor {
    /// Returns a copy of a `w::QTextEdit`'s [`q::QTextCursor`] that represents the currently visible
    /// cursor.
    pub fn get(widget: &w::QTextEdit) -> Self {
        // SAFETY: `widget` has already been dereferenced and `text_cursor` creates a copy
        unsafe { widget.text_cursor().into() }
    }
    /// Sets the visible cursor.
    ///
    /// # Safety
    ///
    /// `widget` must be valid and non-null.
    pub unsafe fn set<T: CastInto<Ptr<w::QTextEdit>>>(&self, widget: T) {
        unsafe { widget.cast_into().set_text_cursor(&self.inner) }
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
    pub fn block(&self) -> QTextBlock {
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
    pub fn convert_to_list(&self, format: &QTextListFormat) -> QTextList<q::QTextCursor> {
        QTextList {
            inner: unsafe { self.inner.create_list_q_text_list_format(&format.inner) },
            _owner: &self.inner,
        }
    }
    /// Returns a pointer to the current frame. Returns `None` if the cursor is invalid.
    pub fn current_frame(&self) -> Option<QTextFrame<q::QTextCursor>> {
        unsafe { nonnull(self.inner.current_frame()) }.map(|frame| QTextFrame {
            inner: frame,
            _owner: &self.inner,
        })
    }
    /// Returns the current list if the cursor position is inside a block that is part of a list;
    /// otherwise returns `None`.
    pub fn current_list(&self) -> Option<QTextList<q::QTextCursor>> {
        unsafe { nonnull(self.inner.current_list()) }.map(|list| QTextList {
            inner: list,
            _owner: &self.inner,
        })
    }
    /// Returns a pointer to the current table if the cursor position is inside a block that is part
    /// of a table; otherwise returns `None`.
    pub fn current_table(&self) -> Option<QTextTable<q::QTextCursor>> {
        unsafe { nonnull(self.inner.current_table()) }.map(|table| QTextTable {
            inner: table,
            _owner: &self.inner,
        })
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
    pub fn document(&self) -> QTextDocument {
        QTextDocument {
            inner: unsafe { self.inner.document() },
        }
    }
    /// Inserts a new empty block at the cursor position with the current [`BlockFormat`].
    pub fn insert_block(&self) {
        unsafe {
            self.inner
                .insert_block_2a(&self.format.block.inner, &self.format.text.inner)
        }
    }
    /// Inserts a new empty block at the cursor position with the given `format`.
    pub fn insert_block_formatted(&self, format: &QTextBlockFormat) {
        unsafe {
            self.inner
                .insert_block_2a(&format.inner, &self.format.text.inner)
        }
    }
    /// Inserts the text `fragment` at the current position.
    pub fn insert_fragment(&self, fragment: &QTextFragment) {
        unsafe { self.inner.insert_fragment(&fragment.inner) }
    }
    /// Inserts a frame with the given `format` at the current cursor position, moves the cursor
    /// position inside the frame, and returns the frame.
    pub fn insert_frame(&self, format: &QTextFrameFormat) -> QTextFrame<q::QTextCursor> {
        QTextFrame {
            inner: unsafe { self.inner.insert_frame(&format.inner) },
            _owner: &self.inner,
        }
    }
    /// Inserts the `text` at the current position. The text is interpreted as HTML.
    pub fn insert_html<S: Printable>(&self, text: S) {
        unsafe { self.inner.insert_html(&text.to_print()) }
    }
    /// Inserts the image defined by the given `format` at the cursor's current position with the
    /// specified `alignment`.
    pub fn insert_image_formatted(&self, format: &QTextImageFormat, alignment: FramePosition) {
        unsafe {
            self.inner
                .insert_image_q_text_image_format_position(&format.inner, alignment)
        }
    }
    /// Convenience method for inserting the image with the given `name` at the current position.
    ///
    /// This uses [`Document::add_resource`].
    pub fn insert_image_resource<S: Printable>(&self, name: S) {
        unsafe { self.inner.insert_image_q_string(&name.to_print()) }
    }
    /// Convenience function for inserting the given `image` with a `name` at the current position.
    pub fn insert_image_named<S: Printable>(&self, image: &QImage, name: S) {
        unsafe {
            self.inner
                .insert_image_q_image_q_string(&image.inner, &name.to_print())
        }
    }
    /// Convenience function for inserting the given `image` at the current position.
    pub fn insert_image(&self, image: &QImage) {
        unsafe { self.inner.insert_image_q_image(&image.inner) }
    }
    /// Inserts a new block at the current position and makes it the first list item of a newly
    /// created list with the given `format`. Returns the created list.
    pub fn insert_list(&self, format: &QTextListFormat) -> QTextList<q::QTextCursor> {
        QTextList {
            inner: unsafe { self.inner.insert_list_q_text_list_format(&format.inner) },
            _owner: &self.inner,
        }
    }
    /// Creates a new table with the given `format` and the given number of `rows` and `columns`,
    /// inserts it at the current cursor position in the document, and returns the table object. The
    /// cursor is moved to the beginning of the first cell.
    ///
    /// There must be at least one row and one column in the tQLineEditable.
    pub fn insert_table(
        &self,
        rows: c_int,
        columns: c_int,
        format: &QTextTableFormat,
    ) -> QTextTable<q::QTextCursor> {
        QTextTable {
            inner: unsafe { self.inner.insert_table_3a(rows, columns, &format.inner) },
            _owner: &self.inner,
        }
    }
    /// Inserts text at the current position, using the current [`CharFormat`].
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text<S: Printable>(&self, text: S) {
        unsafe {
            self.inner
                .insert_text_2a(&text.to_print(), &self.format.text.inner)
        }
    }
    /// Inserts text at the current position, using the given `format`.
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text_formatted<S: Printable>(&self, text: S, format: &QTextCharFormat) {
        unsafe { self.inner.insert_text_2a(&text.to_print(), &format.inner) }
    }
    /// Inserts text at the current position, using the current character format plus text color.
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`insert_block`](Self::insert_block) calls.
    pub fn insert_text_colored<S, B>(&self, text: S, fg: Option<B>, bg: Option<B>)
    where
        S: Printable,
        B: Borrow<QColor>,
    {
        unsafe {
            let fmt = q::QTextCharFormat::new_copy(&self.format.text.inner);
            if let Some(fg) = fg {
                fmt.set_foreground_color(fg.borrow());
            }
            if let Some(bg) = bg {
                fmt.set_background_color(bg.borrow());
            }
            self.inner.insert_text_2a(&text.to_print(), &fmt);
        }
    }
    /// Returns `true` if the cursor is at the end of a block; otherwise returns `false`.
    pub fn is_at_block_end(&self) -> bool {
        unsafe { self.inner.at_block_end() }
    }
    /// Returns `true` if the cursor is at the start of a block; otherwise returns `false`.
    pub fn is_at_block_start(&self) -> bool {
        unsafe { self.inner.at_block_start() }
    }
    /// Returns `true` if the cursor is at the end of the document; otherwise returns `false`.
    pub fn is_at_end(&self) -> bool {
        unsafe { self.inner.at_end() }
    }
    /// Returns `true` if the cursor is at the start of the document; otherwise returns `false`.
    pub fn is_at_start(&self) -> bool {
        unsafe { self.inner.at_start() }
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
            let cursor = q::QTextCursor::new_copy(&self.inner);
            cursor.select(selection);
            Selection { inner: cursor }
        }
    }
    /// Returns the current selection, if one exists.
    pub fn selection(&self) -> Option<Selection> {
        unsafe {
            if !self.inner.has_selection() {
                return None;
            }
            let start = self.inner.selection_start();
            let end = self.inner.selection_end();
            let cursor = q::QTextCursor::new_copy(&self.inner);
            cursor.set_position_2a(start, MoveMode::MoveAnchor);
            cursor.set_position_2a(end, MoveMode::KeepAnchor);
            Some(Selection { inner: cursor })
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
