use std::ops::{Bound, RangeBounds};
use std::os::raw::{c_double, c_int};

use cpp_core::{CppBox, CppDeletable, Ptr};
use qt_core::{CursorMoveStyle, LayoutDirection, QFlags, QPtr, QString, QUrl};
use qt_gui as q;
use qt_gui::q_text_cursor::{MoveMode, MoveOperation};
use qt_gui::q_text_document::{MarkdownFeature, MetaInformation, ResourceType, Stacks};
pub use qt_gui::q_text_frame_format::Position as FramePosition;
pub use qt_gui::q_text_list_format::Style as ListStyle;

use super::format::{QTextBlockFormat, QTextCharFormat};
use super::{if_valid, nonnull, Position};
use crate::graphics::Painter;
use crate::text::{QTextCursor, QTextFrameFormat};
use crate::variant::QVariant;
use crate::{Printable, QFont, QRectF};

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextBlock {
    pub(crate) inner: CppBox<q::QTextBlock>,
}

impl_eq_cpp!(QTextBlock);

impl From<CppBox<q::QTextBlock>> for QTextBlock {
    fn from(value: CppBox<q::QTextBlock>) -> Self {
        Self { inner: value }
    }
}

impl QTextBlock {
    /// Returns the `BlockFormat` that describes block-specific properties.
    pub fn block_format(&self) -> QTextBlockFormat {
        unsafe { self.inner.block_format() }.into()
    }
    /// Returns the number of this block, or `None` if the block is invalid.
    pub fn number(&self) -> Option<c_int> {
        if_valid(unsafe { self.inner.block_number() })
    }
    /// Returns the `CharFormat` that describes the block's character format. The block's character
    /// format is used when inserting text into an empty block.
    pub fn char_format(&self) -> QTextCharFormat {
        unsafe { self.inner.char_format() }.into()
    }
    /// Clears the [`Layout`] that is used to lay out and display the block's contents.
    pub fn clear_layout(&self) {
        unsafe { self.inner.clear_layout() }
    }
    /// Returns `true` if the given position is located within the text block; otherwise returns
    /// `false`.
    pub fn contains(&self, position: Position) -> bool {
        unsafe { self.inner.contains(position) }
    }
    /// Returns the text document this text block belongs to, or `None` if the text block does not
    /// belong to any document.
    pub fn document(&self) -> Option<QTextDocument> {
        unsafe { nonnull(self.inner.document()) }.map(|inner| QTextDocument { inner })
    }
    /// Returns the first line number of this block, or `None` if the block is invalid. If the
    /// layout does not support this feature, the line number is identical to the block number.
    pub fn first_line_number(&self) -> Option<c_int> {
        if_valid(unsafe { self.inner.first_line_number() })
    }
    /// Returns `true` if this text block is valid; otherwise returns `false`.
    pub fn is_valid(&self) -> bool {
        unsafe { self.inner.is_valid() }
    }
    // Returns `true` if the block is visible; otherwise returns `false`.
    pub fn is_visible(&self) -> bool {
        unsafe { self.inner.is_visible() }
    }
    /// Returns the layout that is used to lay out and display the block's contents.
    pub fn layout(&self) -> QTextLayout<q::QTextBlock> {
        QTextLayout {
            inner: unsafe { self.inner.layout() },
            _owner: &self.inner,
        }
    }
    /// Returns the length of the block in characters.
    ///
    /// **Note:** The length returned includes all formatting characters, for example, `\n`.
    pub fn len(&self) -> c_int {
        unsafe { self.inner.length() }
    }
    /// Returns `true` if the block has no contents; otherwise returns `false`.
    pub fn is_empty(&self) -> bool {
        unsafe { self.inner.length() == 0 }
    }
    /// Returns the line count if the document layout supports it.
    pub fn line_count(&self) -> Option<c_int> {
        if_valid(unsafe { self.inner.line_count() })
    }
    /// Returns the index of the block's first character within the document.
    pub fn position(&self) -> Position {
        unsafe { self.inner.position() }
    }
    /// Returns the block's revision.
    pub fn revision(&self) -> c_int {
        unsafe { self.inner.revision() }
    }
    /// Sets a block's revision.
    pub fn set_revision(&self, rev: c_int) {
        unsafe { self.inner.set_revision(rev) }
    }
    /// Stores the specified `state` integer value in the text block. This may be useful for example
    /// in a syntax highlighter to store a text parsing state.
    pub fn set_user_state(&self, state: c_int) {
        unsafe { self.inner.set_user_state(state) }
    }
    /// Sets the block's visibility.
    pub fn set_visible(&self, visible: bool) {
        unsafe { self.inner.set_visible(visible) }
    }
    /// Returns the block's contents as plain text.
    pub fn text(&self) -> String {
        unsafe { self.inner.text().to_std_string() }
    }
    /// Returns the resolved text direction.
    // If the block has no explicit direction set, it will resolve the direction from the blocks
    // content.
    pub fn text_direction(&self) -> LayoutDirection {
        unsafe { self.inner.text_direction() }
    }
    /// If the block represents a list item, returns the list that the item belongs to; otherwise
    /// returns `None`.
    pub fn text_list(&self) -> Option<QTextList<q::QTextBlock>> {
        unsafe { nonnull(self.inner.text_list()) }.map(|list| QTextList {
            inner: list,
            _owner: &self.inner,
        })
    }
    /// Returns the integer value previously set with [`set_user_state`](Self::set_user_state).
    ///
    /// If no user state has been set, it defaults to `-1`.
    pub fn user_state(&self) -> c_int {
        unsafe { self.inner.user_state() }
    }
    /// Returns the text block in the document after this block, or `None` if this is the last one.
    ///
    /// Note that the next block may be in a different frame or table to this block.
    pub fn next(&self) -> Option<Self> {
        unsafe {
            let block = self.inner.next();
            if block.is_valid() {
                Some(Self { inner: block })
            } else {
                None
            }
        }
    }
    /// Returns the text block in the document before this block, or `None` if this is the first
    /// one.
    ///
    /// Note that the previous block may be in a different frame or table to this block.
    pub fn previous(&self) -> Option<Self> {
        unsafe {
            let block = self.inner.previous();
            if block.is_valid() {
                Some(Self { inner: block })
            } else {
                None
            }
        }
    }
}

#[derive(Debug)]
pub struct QTextLayout<'a, T: CppDeletable> {
    #[allow(dead_code)]
    pub(crate) inner: Ptr<q::QTextLayout>,
    pub(crate) _owner: &'a CppBox<T>,
}

#[derive(Debug)]
pub struct QTextList<'a, T: CppDeletable> {
    #[allow(dead_code)]
    pub(crate) inner: QPtr<q::QTextList>,
    pub(crate) _owner: &'a CppBox<T>,
}

#[derive(Debug)]
pub struct QTextFrame<'a, T: CppDeletable> {
    #[allow(dead_code)]
    pub(crate) inner: QPtr<q::QTextFrame>,
    pub(crate) _owner: &'a CppBox<T>,
}

#[derive(Debug)]
pub struct QTextTable<'a, T: CppDeletable> {
    #[allow(dead_code)]
    pub(crate) inner: QPtr<q::QTextTable>,
    pub(crate) _owner: &'a CppBox<T>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextFragment {
    pub(crate) inner: CppBox<q::QTextDocumentFragment>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextOption {
    pub(crate) inner: CppBox<q::QTextOption>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Selection {
    pub(crate) inner: CppBox<q::QTextCursor>,
}

impl_eq_cpp!(Selection);

impl Selection {
    /// Returns the starting position of the selection. Selection boundaries are between characters.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn start(&self) -> c_int {
        unsafe { self.inner.selection_start() }
    }
    /// Returns the ending position of the selection. Selection boundaries are between characters.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn end(&self) -> c_int {
        unsafe { self.inner.selection_end() }
    }
    /// Returns the number of characters in the selection.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn len(&self) -> c_int {
        unsafe { (self.inner.anchor() - self.inner.position()).abs() }
    }
    /// Returns true if there are no characters in the selection; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        unsafe { self.inner.anchor() == self.inner.position() }
    }
    /// Inserts a frame at the current position with the given `format`, moves the selection inside
    /// the frame, and returns the frame.
    pub fn insert_frame(&self, format: QTextFrameFormat) -> QTextFrame<q::QTextCursor> {
        QTextFrame {
            inner: unsafe { self.inner.insert_frame(&format.inner) },
            _owner: &self.inner,
        }
    }
    /// Replaces the contents of the selection with `text`, using the given character `format`.
    ///
    /// Any ASCII linefeed characters `(\n)` in the inserted text are transformed into unicode block
    /// separators, corresponding to [`Cursor::insert_block`] calls.
    pub fn replace<S: Printable>(&self, text: S, format: QTextCharFormat) {
        unsafe { self.inner.insert_text_2a(&text.to_print(), &format.inner) }
    }
    /// Deletes the content of the selection.
    pub fn delete(&self) {
        unsafe { self.inner.remove_selected_text() }
    }
    /// Returns the selection's text. This only returns the text, with no rich text formatting
    /// information. If you want a document fragment (i.e. formatted rich text) use [`fragment`]
    /// instead.
    ///
    /// **Note:** If the selection obtained from an editor spans a line break, the text will contain
    /// a `Unicode U+2029` paragraph separator character instead of a newline `\n` character.
    pub fn text(&self) -> String {
        unsafe { self.inner.selected_text().to_std_string() }
    }
    /// Returns the current selection with all its formatting information. If you just want the
    /// selected text (i.e. plain text) use [`text`] instead.
    ///
    /// Note: May include special unicode characters such as [`QChar::ParagraphSeparator`].
    pub fn fragment(&self) -> QTextFragment {
        QTextFragment {
            inner: unsafe { self.inner.selection() },
        }
    }
    /// Sets the block format of all blocks that are contained in the selection to `format`.
    pub fn set_block_format(&self, format: &QTextBlockFormat) {
        unsafe { self.inner.set_block_format(&format.inner) }
    }
    /// Sets the char format of all blocks that are contained in the selection to `format`.
    pub fn set_char_format(&self, format: &QTextCharFormat) {
        unsafe { self.inner.set_block_char_format(&format.inner) }
    }
    /// Modifies the block format of all blocks that are contained in the selection with the given
    /// `format`.
    pub fn merge_block_format(&self, format: &QTextBlockFormat) {
        unsafe { self.inner.merge_block_format(&format.inner) }
    }
    /// Modifies the char format of all blocks that are contained in the selection with the given
    /// `format`.
    pub fn merge_char_format(&self, format: &QTextCharFormat) {
        unsafe { self.inner.merge_char_format(&format.inner) }
    }
    /// Returns a cursor placed at the start of the selection. This can be used to measure
    /// properties such as column position.
    pub fn start_cursor(&self) -> QTextCursor {
        unsafe {
            let cursor = QTextCursor::from(q::QTextCursor::new_copy(&self.inner));
            cursor.set_position(self.inner.selection_start());
            cursor
        }
    }
    /// Returns a cursor placed at the end of the selection. This can be used to measure
    /// properties such as column position.
    pub fn end_cursor(&self) -> QTextCursor {
        unsafe {
            let cursor = QTextCursor::from(q::QTextCursor::new_copy(&self.inner));
            cursor.set_position(self.inner.selection_end());
            cursor
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextDocumentLayout {
    pub(crate) inner: QPtr<q::QAbstractTextDocumentLayout>,
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QTextDocument {
    pub(crate) inner: QPtr<q::QTextDocument>,
}

impl QTextDocument {
    /// Adds a `resource` to the resource cache, using `resource_type` and `name` as identifiers.
    ///
    /// Note: below, `cursor` is a [`Cursor`](super::cursor::Cursor).
    ///
    ///```
    /// use qt_gui::q_text_document::ResourceType;
    /// # let (argc, argv) = qt_core::QCoreApplicationArgs::new().get();
    /// # let app = unsafe { qt_widgets::QApplication::new_2a(argc, argv) };
    /// # let image = qmushclient::binding::variant::RVariant::from(unsafe { qt_core::QVariant::new() });
    /// # let cursor = unsafe { qmushclient::binding::text::Cursor::get(qt_widgets::QTextEdit::new().into_ptr()) };
    ///
    /// let document = cursor.document();
    ///
    /// // Add an image as a resource in order to reference it from within the document:
    /// document.add_resource(ResourceType::ImageResource, "mydata://image.png", image);
    ///
    /// // Insert the image into the document using the `Cursor` API:
    /// cursor.insert_image_resource("mydata://image.png");
    ///
    /// // Alternatively, insert images using the HTML `img` tag:
    /// cursor.insert_html(r#"<img src="mydata://image.png" />"#);
    ///```
    pub fn add_resource<T: Into<QVariant>>(
        &self,
        resource_type: ResourceType,
        name: &str,
        resource: T,
    ) {
        unsafe {
            self.inner.add_resource(
                resource_type.to_int(),
                &QUrl::new_1a(&QString::from_std_str(name)),
                &resource.into().inner,
            )
        }
    }
    /// Returns data of the specified `resource_type` from the resource with the given `name`.
    ///
    /// This function is called by the rich text engine to request data that isn't directly stored
    /// by the document, but still associated with it. For example, images are referenced indirectly
    /// by the name attribute of an [`ImageFormat`](super::format::ImageFormat) object.
    pub fn resource<T: From<QVariant>>(&self, resource_type: ResourceType, name: &str) -> Option<T> {
        QVariant::from(unsafe {
            self.inner.resource(
                resource_type.to_int(),
                &QUrl::new_1a(&QString::from_std_str(name)),
            )
        })
        .try_into()
        .ok()
    }
    /// Returns data of the specified `resource_type` from the source with the given `name`.
    ///
    /// This function is called by the rich text engine to request data that isn't directly stored
    /// by the document, but still associated with it. For example, images are referenced indirectly
    /// by the name attribute of an [`ImageFormat`](super::format::ImageFormat) object.
    ///
    /// Resources are cached internally in the document. If a resource can not be found in the
    /// cache, Adjusts the document to a reasonable size.
    pub fn adjust_size(&self) {
        unsafe { self.inner.adjust_size() }
    }
    /// Returns the number of available redo steps.
    pub fn available_redo_steps(&self) -> c_int {
        unsafe { self.inner.available_redo_steps() }
    }
    /// Returns the number of available undo steps.
    pub fn available_undo_steps(&self) -> c_int {
        unsafe { self.inner.available_undo_steps() }
    }
    /// Returns the 16-bit character at the given `position`, or `None` if out of bounds.
    pub fn get(&self, position: c_int) -> Option<u16> {
        unsafe {
            let c = self.inner.character_at(position).unicode();
            if c == 0 {
                None
            } else {
                Some(c)
            }
        }
    }
    /// Returns the number of characters in this document.
    ///
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn len(&self) -> c_int {
        unsafe { self.inner.character_count() }
    }
    /// Clears the document.
    pub fn clear(&self) {
        unsafe { self.inner.clear() }
    }
    /// Clears the given `stacks`.
    ///
    /// This method clears any commands on the undo stack, the redo stack, or both (the default).
    pub fn clear_undo_redo_stacks(&self, stacks: Stacks) {
        unsafe { self.inner.clear_undo_redo_stacks_1a(stacks) }
    }
    /// The default cursor movement style used by all [`Cursor`](super::cursor::Cursor) objects
    /// created from this document. The default is [`CursorMoveStyle::LogicalMoveStyle`].
    pub fn default_cursor_move_style(&self) -> CursorMoveStyle {
        unsafe { self.inner.default_cursor_move_style() }
    }
    /// Sets the default cursor movement `style` used by all [`Cursor`](super::cursor::Cursor)
    /// objects created from this document.
    pub fn set_default_cursor_move_style(&self, style: CursorMoveStyle) {
        unsafe { self.inner.set_default_cursor_move_style(style) }
    }
    /// Returns the default font to be used in the document layout.
    pub fn default_font(&self) -> QFont {
        QFont::from(unsafe { self.inner.default_font() })
    }
    /// Sets the default `font` to be used in the document layout.
    pub fn set_default_font(&self, font: &QFont) {
        unsafe { self.inner.set_default_font(&font.inner) }
    }
    /// The default text option used on all layout objects in the document. This allows setting
    /// global properties for the document such as the default word wrap mode.
    pub fn default_text_option(&self) -> QTextOption {
        QTextOption {
            inner: unsafe { self.inner.default_text_option() },
        }
    }
    /// Sets the default text `option` used on all layout objects in the document. This allows
    /// setting global properties for the document such as the default word wrap mode.
    pub fn set_default_text_option(&self, option: &QTextOption) {
        unsafe { self.inner.set_default_text_option(&option.inner) }
    }
    /// Returns the document layout for this document.
    pub fn layout(&self) -> QTextDocumentLayout {
        QTextDocumentLayout {
            inner: unsafe { self.inner.document_layout() },
        }
    }
    /// Sets the document layout for this document. The previous layout is deleted.
    pub fn set_layout(&self, layout: &QTextDocumentLayout) {
        unsafe { self.inner.set_document_layout(&layout.inner) }
    }
    /// Draws the content of the document with `painter`.
    pub fn draw_contents(&self, painter: &Painter) {
        unsafe { self.inner.draw_contents_1a(&painter.inner) }
    }
    /// Draws the content of the document with `painter`, clipped to `rect`.
    pub fn draw_contents_in(&self, painter: &Painter, rect: QRectF) {
        unsafe {
            self.inner
                .draw_contents_2a(&painter.inner, &CppBox::<qt_core::QRectF>::from(rect))
        }
    }
    /// Returns the block that contains the `pos`-th character, or `None` if the character is out of
    /// bounds.
    pub fn block_at(&self, pos: c_int) -> Option<QTextBlock> {
        unsafe {
            let block = self.inner.find_block(pos);
            if block.is_valid() {
                Some(QTextBlock { inner: block })
            } else {
                None
            }
        }
    }
    /// Returns the document's first text block.
    pub fn first_block(&self) -> QTextBlock {
        QTextBlock {
            inner: unsafe { self.inner.first_block() },
        }
    }
    /// Returns the document's last text block.
    pub fn last_block(&self) -> QTextBlock {
        QTextBlock {
            inner: unsafe { self.inner.last_block() },
        }
    }
    /// Returns the ideal width of the text document. The ideal width is the actually used width of
    /// the document without optional alignments taken into account. It is always less than or equal
    /// to the width of `size`.
    pub fn ideal_width(&self) -> c_double {
        unsafe { self.inner.ideal_width() }
    }
    /// The width used for text list and text block indenting. The default indent wdth is 40.
    pub fn indent_width(&self) -> c_double {
        unsafe { self.inner.indent_width() }
    }
    /// Sets the `width` used for text list and text block indenting.
    ///
    /// The indent properties of [`ListFormat`](super::format::ListFormat) and [`BlockFormat`]
    /// specify multiples of this value.
    pub fn set_indent_width(&self, width: c_double) {
        unsafe { self.inner.set_indent_width(width) }
    }
    /// Returns true if the document is empty; otherwise returns false.
    pub fn is_empty(&self) -> bool {
        unsafe { self.inner.is_empty() }
    }
    /// Returns `true` if a redo is available; otherwise returns `false`.
    pub fn is_redo_available(&self) -> bool {
        unsafe { self.inner.is_redo_available() }
    }
    /// Returns `true` if an undo is available; otherwise returns `false`.
    pub fn is_undo_available(&self) -> bool {
        unsafe { self.inner.is_undo_available() }
    }
    /// Returns the number of blocks in this document.
    pub fn block_count(&self) -> c_int {
        unsafe { self.inner.block_count() }
    }
    /// Returns the number of lines in this document. If the layout does not support this, returns
    /// [`block_count`](Self::block_count).
    pub fn line_count(&self) -> c_int {
        unsafe { self.inner.line_count() }
    }
    /// Marks the contents specified by the given `position` and `length` as "dirty", informing the
    /// document that it needs to be laid out again.
    pub fn mark_contents_dirty(&self, position: c_int, length: c_int) {
        unsafe { self.inner.mark_contents_dirty(position, length) }
    }
    /// Returns meta information about the document of the type specified by `info`.
    pub fn meta_information(&self, info: MetaInformation) -> String {
        unsafe { self.inner.meta_information(info).to_std_string() }
    }
    /// Sets the document's meta information of the type specified by `info` to the given `string`.
    pub fn set_meta_information(&self, info: MetaInformation, string: &str) {
        unsafe {
            self.inner
                .set_meta_information(info, &QString::from_std_str(string))
        }
    }
    /// Returns the number of pages in this document.
    pub fn page_count(&self) -> c_int {
        unsafe { self.inner.page_count() }
    }
    /// Returns the document's revision if undo is enabled.
    ///
    /// The revision is guaranteed to increase when a document that is not modified is edited.
    pub fn revision(&self) -> c_int {
        unsafe { self.inner.revision() }
    }
    /// Returns a string containing an HTML representation of the document.
    pub fn to_html(&self) -> String {
        unsafe { self.inner.to_html_0a().to_std_string() }
    }
    /// Replaces the entire contents of the document with the given HTML-formatted `text`. Ths
    /// resets the undo/redo history.
    ///
    /// The HTML formatting is respected as much as possible; for example, "<b>bold</b> text" will
    /// produce text where the first word has a font weight that gives it a bold appearance:
    /// "**bold** text".
    ///
    /// **Note:** It is the responsibility of the caller to make sure that the text is correctly
    /// decoded.
    pub fn set_html(&self, text: &str) {
        unsafe { self.inner.set_html(&QString::from_std_str(text)) }
    }
    /// Returns a string containing a Markdown representation of the document with the given
    /// `features`, or `None` if writing fails for any reason.
    pub fn to_markdown(&self, features: QFlags<MarkdownFeature>) -> Option<String> {
        unsafe {
            if self.inner.is_empty() {
                return Some(String::new());
            }
            let markdown = self.inner.to_markdown_1a(features).to_std_string();
            if markdown.is_empty() {
                None
            } else {
                Some(markdown)
            }
        }
    }
    /// Replaces the entire contents of the document with the given Markdown-formatted `text`, with
    /// the given `features` supported. By default, all supported GitHub-style Markdown features are
    /// included; pass [`MarkdownFeature::MarkdownDialectCommonMark`] for a more basic parse.
    ///
    /// The Markdown formatting is respected as much as possible; for example, "\*bold\*" text will
    /// produce text where the first word has a font weight that gives it an emphasized appearance.
    ///
    /// Parsing of HTML included in the `text` is handled the same way as in
    /// [`set_html`](Self::set_html); however, Markdown formatting inside HTML blocks is not
    /// supported.
    ///
    /// Some features of the parser can be enabled or disabled via the `features` argument:
    ///
    /// - [`MarkdownFeature::MarkdownNoHTML`]: Any HTML tags in the Markdown text will be discarded.
    /// - [`MarkdownFeature::MarkdownDialectCommonMark`]: The parser supprts only the features standardized by [CommonMark](https://commonmark.org/).
    /// - [`MarkdownFeature::MarkdownDialectGitHub`]: The parser supports the [GitHub dialect](https://github.github.com/gfm/).
    ///
    /// The default is [`MarkdownFeature::MarkdownDialectGitHub`].
    ///
    /// The undo/redo history is reset when this function is called.
    pub fn set_markdown(&self, text: &str, features: QFlags<MarkdownFeature>) {
        unsafe {
            self.inner
                .set_markdown_2a(&QString::from_std_str(text), features)
        }
    }
    /// Returns the plain text contained in the document.
    ///
    /// This function returns the same as [`to_raw_text`](Self::to_raw_text), but will replace some
    /// unicode characters with ASCII alternatives. In particular, no-break space (U+00A0) is
    /// replaced by a regular space (U+0020), and both paragraph (U+2029) and line (U+2028)
    /// separators are replaced by line feed (U+000A). If you need the precise contents of the
    /// document, use `to_raw_text` instead.
    ///
    /// **Note:** Embedded objects, such as images, are represented by a Unicode value U+FFFC
    /// (OBJECT REPLACEMENT CHARACTER).
    pub fn to_plain_text(&self) -> String {
        unsafe { self.inner.to_plain_text().to_std_string() }
    }
    /// Returns the raw text contained in the document without any formatting information.
    pub fn to_raw_text(&self) -> String {
        unsafe { self.inner.to_raw_text().to_std_string() }
    }
    /// Replaces the ntire contents of the document with the given plain `text`.
    ///
    /// The undo/redo history is reset when this function is called.
    pub fn set_plain_text(&self, text: &str) {
        unsafe { self.inner.set_plain_text(&QString::from_std_str(text)) }
    }
    /// **Note:** The "characters" in this case refer to the string of [`QChar`](qt_core::QChar)
    /// objects, i.e. 16-bit Unicode characters, and the position is considered an index into this
    /// string. This does not necessarily correspond to individual graphemes in the writing system,
    /// as a single grapheme may be represented by multiple Unicode characters, such as in the case
    /// of surrogate pairs, linguistic ligatures or diacritics.
    pub fn select<R: RangeBounds<c_int>>(&self, range: R) -> Selection {
        unsafe {
            let cursor = q::QTextCursor::from_q_text_document(&self.inner);
            match range.start_bound() {
                Bound::Included(i) => cursor.set_position_2a(*i, MoveMode::MoveAnchor),
                Bound::Excluded(i) => cursor.set_position_2a(*i + 1, MoveMode::MoveAnchor),
                Bound::Unbounded => (),
            }
            match range.end_bound() {
                Bound::Included(i) => cursor.set_position_2a(*i, MoveMode::KeepAnchor),
                Bound::Excluded(i) => cursor.set_position_2a(*i + 1, MoveMode::KeepAnchor),
                Bound::Unbounded => {
                    cursor.move_position_2a(MoveOperation::End, MoveMode::KeepAnchor);
                }
            }
            Selection { inner: cursor }
        }
    }
}
