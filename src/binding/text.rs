use std::borrow::Borrow;
use std::os::raw::c_int;

use cpp_core::{CppBox, Ptr, StaticUpcast};
use qt_core::{LayoutDirection, QObject, QPtr, QString};
use qt_gui::q_font::Weight;
use qt_gui::q_text_cursor::{MoveMode, MoveOperation, SelectionType};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;
use qt_gui::*;
use qt_widgets::QTextEdit;

use super::color::{Colored, RColor};
use super::{Binding, Printable, RImage};
use crate::binding::RFont;
pub type Position = c_int;

/// # Safety
///
/// `ptr` must be valid.
unsafe fn from_nullable<Q: StaticUpcast<QObject>, To: From<QPtr<Q>>>(ptr: QPtr<Q>) -> Option<To> {
    if unsafe { ptr.is_null() } {
        None
    } else {
        Some(ptr.into())
    }
}

#[derive(Debug)]
pub struct Formats {
    pub block: BlockFormat,
    pub text: CharFormat,
    pub list: ListFormat,
    pub frame: FrameFormat,
    pub table: TableFormat,
    pub image: ImageFormat,
}

#[derive(Debug)]
pub struct Cursor {
    inner: CppBox<QTextCursor>,
    pub format: Formats,
}

impl From<CppBox<QTextCursor>> for Cursor {
    fn from(inner: CppBox<QTextCursor>) -> Self {
        unsafe {
            let format = Formats {
                block: inner.block_format().into(),
                text: inner.char_format().into(),
                list: QTextListFormat::new().into(),
                frame: QTextFrameFormat::new().into(),
                table: QTextTableFormat::new().into(),
                image: QTextImageFormat::new().into(),
            };
            Self { inner, format }
        }
    }
}

impl Binding for Cursor {
    type Bind = CppBox<QTextCursor>;

    fn into_raw(self) -> Self::Bind {
        self.inner
    }
}

impl Cursor {
    /// Returns a copy of the Cursor that represents the currently visible cursor. Note that changes on the returned cursor do not affect the widget's cursor; use set_text_cursor() to update the visible cursor.
    ///
    /// # Safety
    ///
    /// `widget` must be valid.
    pub unsafe fn get<T: StaticUpcast<QTextEdit> + StaticUpcast<QObject>>(widget: &QPtr<T>) -> Self {
        unsafe {
            let widget: QPtr<QTextEdit> = widget.static_upcast();
            widget.text_cursor().into()
        }
    }
    /// Sets the visible cursor.
    ///
    /// # Safety
    ///
    /// `widget` must be valid.
    pub unsafe fn set<T: StaticUpcast<QTextEdit> + StaticUpcast<QObject>>(&self, widget: &QPtr<T>) {
        unsafe {
            let widget: QPtr<QTextEdit> = widget.static_upcast();
            widget.set_text_cursor(&self.inner);
        }
    }
    /// Returns the anchor position; this is the same as position() unless there is a selection in which case position() marks one end of the selection and anchor() marks the other end. Just like the cursor position, the anchor position is between characters.
    pub fn anchor(&self) -> Position {
        unsafe { self.inner.anchor() }
    }
    /// Returns true if the cursor is at the end of a block; otherwise returns false.
    pub fn at_block_end(&self) -> bool {
        unsafe { self.inner.at_block_end() }
    }
    /// Returns true if the cursor is at the start of a block; otherwise returns false.
    pub fn at_block_start(&self) -> bool {
        unsafe { self.inner.at_block_start() }
    }
    /// Returns true if the cursor is at the end of the document; otherwise returns false.
    pub fn at_end(&self) -> bool {
        unsafe { self.inner.at_end() }
    }
    /// Returns true if the cursor is at the start of the document; otherwise returns false.
    pub fn at_start(&self) -> bool {
        unsafe { self.inner.at_start() }
    }
    /// Indicates a block of editing operations on the document that should appear as a single operation from an undo/redo point of view.
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
    pub fn current_block(&self) -> Block {
        unsafe { self.inner.block() }.into()
    }
    /// Clears the current selection by setting the anchor to the cursor position.
    /// Note that it does not delete the text of the selection.
    pub fn deselect(&self) {
        unsafe {
            self.inner.clear_selection();
        }
    }
    /// Returns the position of the cursor within its containing line.
    /// Note that this is the column number relative to a wrapped line, not relative to the block (i.e. the paragraph).
    pub fn column(&self) -> c_int {
        unsafe { self.inner.column_number() }
    }
    /// Creates and returns a new list with the given style, making the cursor's current paragraph the first list item.
    pub fn convert_to_list_style(&self, style: ListStyle) -> List {
        unsafe { self.inner.create_list_style(style) }.into()
    }
    /// Creates and returns a new list with the given format, and makes the current paragraph the cursor is in the first list item.
    pub fn convert_to_list(&self) -> List {
        unsafe {
            self.inner
                .create_list_q_text_list_format(&self.format.list.0)
        }
        .into()
    }
    /// Returns a pointer to the current frame. Returns None if the cursor is invalid.
    pub fn current_frame(&self) -> Option<Frame> {
        unsafe { from_nullable(self.inner.current_frame()) }
    }
    /// Returns the current list if the cursor position() is inside a block that is part of a list; otherwise returns None.
    pub fn current_list(&self) -> Option<List> {
        unsafe { from_nullable(self.inner.current_list()) }
    }
    /// Returns a pointer to the current table if the cursor position() is inside a block that is part of a table; otherwise returns None.
    pub fn current_table(&self) -> Option<Table> {
        unsafe { from_nullable(self.inner.current_table()) }
    }
    /// If there is no selected text, deletes the character at the current cursor position; otherwise deletes the selected text.
    pub fn delete_char(&self) {
        unsafe {
            self.inner.delete_char();
        }
    }
    /// If there is no selected text, deletes the character before the current cursor position; otherwise deletes the selected text.
    pub fn delete_previous_char(&self) {
        unsafe {
            self.inner.delete_previous_char();
        }
    }
    /// Returns the document this cursor is associated with.
    pub fn document(&self) -> Document {
        unsafe { self.inner.document() }.into()
    }
    /// Returns true if the cursor contains a selection; otherwise returns false.
    pub fn has_selection(&self) -> bool {
        unsafe { self.inner.has_selection() }
    }
    /// Inserts a new empty block at the cursor position() with the current block_format() and char_format().
    pub fn insert_block(&self) {
        unsafe {
            self.inner
                .insert_block_2a(&self.format.block.0, &self.format.text.0)
        }
    }
    /// Inserts the text fragment at the current position().
    pub fn insert_fragment(&self, fragment: Fragment) {
        unsafe { self.inner.insert_fragment(&fragment.into_raw()) }
    }
    /// Inserts a frame with the given format at the current cursor position(), moves the cursor position() inside the frame, and returns the frame.
    /// If the cursor holds a selection, the whole selection is moved inside the frame.
    pub fn insert_frame(&self) -> Frame {
        unsafe { self.inner.insert_frame(&self.format.frame.0) }.into()
    }
    /// Inserts the text html at the current position(). The text is interpreted as HTML.
    /// Note: When using this function with a style sheet, the style sheet will only apply to the current block in the document. In order to apply a style sheet throughout a document, use Document::set_default_style_sheet() instead.
    pub fn insert_html<P: Printable>(&self, s: P) {
        unsafe { self.inner.insert_html(&s.to_print()) }
    }
    /// Inserts the image defined by the given format at the cursor's current position with the specified alignment.
    pub fn insert_image_frame(&self, align: FramePosition) {
        unsafe {
            self.inner
                .insert_image_q_text_image_format_position(&self.format.image.0, align)
        }
    }
    /// Convenience method for inserting the image with the given name at the current position().
    /// This uses Document::add_resource().
    pub fn insert_image_resource<P: Printable>(&self, s: P) {
        unsafe { self.inner.insert_image_q_string(&s.to_print()) }
    }
    /// Convenience function for inserting the given image with a name at the current position().
    pub fn insert_image_named<P: Printable>(&self, image: RImage, name: P) {
        unsafe {
            self.inner
                .insert_image_q_image_q_string(&image.into_raw(), &name.to_print())
        }
    }
    /// Convenience function for inserting the given image with an optional name at the current position().
    pub fn insert_image(&self, image: RImage) {
        unsafe { self.inner.insert_image_q_image(&image.into_raw()) }
    }
    /// Inserts a new block at the current position and makes it the first list item of a newly created list with the given format. Returns the created list.
    pub fn insert_list_styled(&self, style: ListStyle) -> List {
        unsafe { self.inner.insert_list_style(style) }.into()
    }
    /// Inserts a new block at the current position and makes it the first list item of a newly created list with the given format. Returns the created list.
    pub fn insert_list(&self) -> List {
        unsafe {
            self.inner
                .insert_list_q_text_list_format(&self.format.list.0)
        }
        .into()
    }
    /// Creates a new table with the given number of rows and columns, inserts it at the current cursor position() in the document, and returns the table object. The cursor is moved to the beginning of the first cell.
    /// There must be at least one row and one column in the table.
    pub fn insert_table(&self, rows: c_int, columns: c_int) -> Table {
        unsafe {
            self.inner
                .insert_table_3a(rows, columns, &self.format.table.0)
        }
        .into()
    }
    /// Inserts text at the current position, using the current character format.
    /// If there is a selection, the selection is deleted and replaced.
    /// Any ASCII linefeed characters (\n) in the inserted text are transformed into unicode block separators, corresponding to insert_block() calls.
    pub fn insert_text<P: Printable>(&self, s: P) {
        unsafe {
            self.inner
                .insert_text_2a(&s.to_print(), &self.format.text.0)
        }
    }
    /// Inserts text at the current position, using a specified character format.
    /// If there is a selection, the selection is deleted and replaced.
    /// Any ASCII linefeed characters (\n) in the inserted text are transformed into unicode block separators, corresponding to insert_block() calls.
    pub fn insert_text_formatted<P: Printable>(&self, s: P, format: &CharFormat) {
        unsafe { self.inner.insert_text_2a(&s.to_print(), &format.0) }
    }
    /// Inserts text at the current position, using the current character format plus text color.
    /// If there is a selection, the selection is deleted and replaced.
    /// Any ASCII linefeed characters (\n) in the inserted text are transformed into unicode block separators, corresponding to insert_block() calls.
    pub fn insert_text_colored<P, B>(&self, s: P, fg: Option<B>, bg: Option<B>)
    where
        P: Printable,
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
            self.inner.insert_text_2a(&s.to_print(), &fmt);
        }
    }
    /// Moves the cursor by performing the given operation n times, using the specified mode, and returns true if all operations were completed successfully; otherwise returns false.
    /// For example, if this function is repeatedly used to seek to the end of the next word, it will eventually fail when the end of the document is reached.
    /// If mode is KeepAnchor, the cursor selects the text it moves over. This is the same effect that the user achieves when they hold down the Shift key and move the cursor with the cursor keys.
    pub fn reposition(&self, op: MoveOperation, mode: MoveMode, n: c_int) -> bool {
        unsafe { self.inner.move_position_3a(op, mode, n) }
    }
    /// Returns the absolute position of the cursor within the document. The cursor is positioned between characters.
    /// Note: The "characters" in this case refer to the string of QChar objects, i.e. 16-bit Unicode characters, and the position is considered an index into this string. This does not necessarily correspond to individual graphemes in the writing system, as a single grapheme may be represented by multiple Unicode characters, such as in the case of surrogate pairs, linguistic ligatures or diacritics.
    pub fn position(&self) -> Position {
        unsafe { self.inner.position() }
    }
    /// Returns the relative position of the cursor within the block. The cursor is positioned between characters.
    /// This is equivalent to position() - block().position().
    /// Note: The "characters" in this case refer to the string of QChar objects, i.e. 16-bit Unicode characters, and the position is considered an index into this string. This does not necessarily correspond to individual graphemes in the writing system, as a single grapheme may be represented by multiple Unicode characters, such as in the case of surrogate pairs, linguistic ligatures or diacritics.
    pub fn position_in_block(&self) -> Position {
        unsafe { self.inner.position_in_block() }
    }
    /// If there is a selection, its content is deleted; otherwise does nothing.
    pub fn delete_selection(&self) {
        unsafe { self.inner.remove_selected_text() }
    }
    /// Selects text in the document according to the given selection.
    pub fn select(&self, selection_type: SelectionType) {
        unsafe { self.inner.select(selection_type) }
    }
    /// Returns the current selection's text (which may be empty). This only returns the text, with no rich text formatting information. If you want a document fragment (i.e. formatted rich text) use selection() instead.
    pub fn selected_text(&self) -> String {
        unsafe { self.inner.selected_text() }
            .to_std_string()
            .replace('\u{2029}', "\n")
    }
    /// Returns the current selection (which may be empty) with all its formatting information. If you just want the selected text (i.e. plain text) use get_selection() instead.
    /// Note: May include special unicode characters such as QChar::ParagraphSeparator.
    pub fn selection(&self) -> Fragment {
        unsafe { self.inner.selection() }.into()
    }
    /// Returns the end of the selection or position() if the cursor doesn't have a selection.
    pub fn selection_end(&self) -> Position {
        unsafe { self.inner.selection_end() }
    }
    /// Returns the start of the selection or position() if the cursor doesn't have a selection.
    pub fn selection_start(&self) -> Position {
        unsafe { self.inner.selection_start() }
    }
    /// Sets the block char format of the current block (or all blocks that are contained in the selection) to format.
    pub fn set_block_char_format(&self, format: &CharFormat) {
        unsafe { self.inner.set_block_char_format(&format.0) }
    }
    /// Moves the cursor to the absolute position in the document specified by pos using a MoveMode specified by m. The cursor is positioned between characters.
    /// The "characters" in this case refer to the string of QChar objects, i.e. 16-bit Unicode characters, and pos is considered an index into this string. This does not necessarily correspond to individual graphemes in the writing system, as a single grapheme may be represented by multiple Unicode characters, such as in the case of surrogate pairs, linguistic ligatures or diacritics. For a more generic approach to navigating the document, use move_position(), which will respect the actual grapheme boundaries in the text.
    pub fn set_position(&self, pos: Position, mode: MoveMode) {
        unsafe { self.inner.set_position_2a(pos, mode) }
    }
    /// Sets the visual x position for vertical cursor movements to x.
    /// The vertical movement x position is cleared automatically when the cursor moves horizontally, and kept unchanged when the cursor moves vertically. The mechanism allows the cursor to move up and down on a visually straight line with proportional fonts, and to gently "jump" over short lines.
    pub fn set_vertical_movement_x(&self, x: Position) {
        unsafe { self.inner.set_vertical_movement_x(x) }
    }
    /// Unsets the visual x position for vertical cursor movements to x.
    /// It will then be set automatically the next time the cursor moves up or down.
    pub fn unset_vertical_movement_x(&self) {
        unsafe { self.inner.set_vertical_movement_x(-1) }
    }
    /// Swaps this text cursor instance with other. This function is very fast and never fails.
    pub fn swap(&self, other: &Self) {
        unsafe { self.inner.swap(&other.inner) }
    }
    /// Returns the visual x position for vertical cursor movements.
    /// A value of None indicates no predefined x position. It will then be set automatically the next time the cursor moves up or down.
    pub fn vertical_movement_x(&self) -> Option<Position> {
        match unsafe { self.inner.vertical_movement_x() } {
            -1 => None,
            x => Some(x),
        }
    }
}

#[derive(Debug, Binding)]
pub struct Block(CppBox<QTextBlock>);

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
        unsafe { from_nullable(self.0.document()) }
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
        unsafe { self.0.layout() }.into()
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
        unsafe { from_nullable(self.0.text_list()) }
    }
    /// Returns the integer value previously set with setUserState() or -1.
    pub fn user_state(&self) -> c_int {
        unsafe { self.0.user_state() }
    }
}

macro_rules! impl_fmt {
    ($t:ty) => {
        impl Colored for $t {
            fn foreground_color(&self) -> RColor {
                self.0.foreground_color()
            }
            fn background_color(&self) -> RColor {
                self.0.background_color()
            }
            fn set_foreground_color(&self, color: &RColor) {
                self.0.set_foreground_color(color)
            }
            fn set_background_color(&self, color: &RColor) {
                self.0.set_background_color(color)
            }
        }
    };
}

#[derive(Debug, Binding)]
pub struct BlockFormat(CppBox<QTextBlockFormat>);
impl_fmt!(BlockFormat);

#[derive(Debug, Binding)]
pub struct CharFormat(CppBox<QTextCharFormat>);
impl_fmt!(CharFormat);

impl CharFormat {
    pub fn set_font(&self, font: &RFont) {
        unsafe {
            self.0.set_font_1a(font);
        }
    }
    pub fn set_bold(&self, enable: bool) {
        unsafe {
            self.0
                .set_font_weight(if enable { Weight::Bold } else { Weight::Normal }.to_int());
        }
    }
    pub fn set_italic(&self, enable: bool) {
        unsafe {
            self.0.set_font_italic(enable);
        }
    }
    pub fn set_strikeout(&self, enable: bool) {
        unsafe {
            self.0.set_font_strike_out(enable);
        }
    }
    pub fn set_underline(&self, enable: bool) {
        unsafe {
            self.0.set_font_underline(enable);
        }
    }

    pub fn set_anchor(&self, enable: bool) {
        unsafe {
            self.0.set_anchor(enable);
        }
    }

    pub fn set_anchor_href(&self, href: &str) {
        unsafe {
            self.0.set_anchor_href(&QString::from_std_str(href));
        }
    }

    pub fn clear_anchor_href(&self) {
        unsafe {
            self.0.set_anchor_href(&QString::new());
        }
    }

    pub fn set_anchor_name(&self, name: &str) {
        unsafe {
            self.0.set_anchor_name(&QString::from_std_str(name));
        }
    }

    pub fn clear_anchor_name(&self) {
        unsafe {
            self.0.set_anchor_name(&QString::new());
        }
    }

    pub fn set_tooltip(&self, tooltip: &str) {
        unsafe {
            self.0.set_tool_tip(&QString::from_std_str(tooltip));
        }
    }

    pub fn clear_tooltip(&self) {
        unsafe {
            self.0.set_tool_tip(&QString::new()); // TODO does this actually
                                                  // work?
        }
    }
}

#[derive(Debug, Binding)]
pub struct Layout(Ptr<QTextLayout>);

#[derive(Debug, Binding)]
pub struct List(QPtr<QTextList>);

#[derive(Debug, Binding)]
pub struct ListFormat(CppBox<QTextListFormat>);
impl_fmt!(ListFormat);

#[derive(Debug, Binding)]
pub struct Frame(QPtr<QTextFrame>);

#[derive(Debug, Binding)]
pub struct FrameFormat(CppBox<QTextFrameFormat>);
impl_fmt!(FrameFormat);

#[derive(Debug, Binding)]
pub struct Table(QPtr<QTextTable>);

#[derive(Debug, Binding)]
pub struct TableFormat(CppBox<QTextTableFormat>);
impl_fmt!(TableFormat);

#[derive(Debug, Binding)]
pub struct Document(QPtr<QTextDocument>);

#[derive(Debug, Binding)]
pub struct Fragment(CppBox<QTextDocumentFragment>);

#[derive(Debug, Binding)]
pub struct ImageFormat(CppBox<QTextImageFormat>);
impl_fmt!(ImageFormat);
