use qt_core::{QBox, QFlags, TextFormat, TextInteractionFlag};
use qt_widgets as q;
use qt_widgets::q_message_box::Icon;

use crate::Printable;

qt_binding!(
    MessageBoxBinding,
    q::QMessageBox,
    super::dialog::DialogBinding
);

impl MessageBoxBinding {
    /// Text displayed in the details area.
    pub fn detailed_text(&self) -> String {
        unsafe { self.inner.detailed_text().to_std_string() }
    }

    /// Sets text to be displayed in the details area.
    pub fn set_detailed_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_detailed_text(&text.to_print()) }
    }

    /// The message box's icon.
    /// The pixmap used to display the actual icon depends on the current GUI style.
    pub fn icon(&self) -> Icon {
        unsafe { self.inner.icon() }
    }

    /// Sets the message box's icon.
    pub fn set_icon(&self, icon: Icon) {
        unsafe { self.inner.set_icon(icon) }
    }

    /// Informative text that can be used to expand upon the text() to give more information to the
    /// user. On the Mac, this text appears in small system font below the text(). On other
    /// platforms, it is simply appended to the existing text.
    pub fn informative_text(&self) -> String {
        unsafe { self.inner.informative_text().to_std_string() }
    }

    /// Sets informative text to provide a fuller description for the message.
    pub fn set_informative_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_informative_text(&text.to_print()) }
    }

    /// Text displayed in the message box.
    pub fn text(&self) -> String {
        unsafe { self.inner.text().to_std_string() }
    }

    /// Sets message box text to display. The text will be interpreted either as a plain text or as
    /// rich text, depending on the text format setting (QMessageBox::textFormat). The default
    /// setting is Qt::AutoText (the message box will try to auto-detect the format of the text).
    pub fn set_text<S: Printable>(&self, text: S) {
        unsafe { self.inner.set_text(&text.to_print()) }
    }

    /// Format of the text displayed by the message box.
    pub fn text_format(&self) -> TextFormat {
        unsafe { self.inner.text_format() }
    }

    /// Sets the format of the text displayed by the message box.
    pub fn set_text_format(&self, format: TextFormat) {
        unsafe { self.inner.set_text_format(format) }
    }

    /// How the label of the message box should interact with user input.
    pub fn text_interaction_flags(&self) -> QFlags<TextInteractionFlag> {
        unsafe { self.inner.text_interaction_flags() }
    }

    /// Configure how the label of the message box should interact with user input.
    pub fn set_text_interaction_flags(&self, flags: QFlags<TextInteractionFlag>) {
        unsafe { self.inner.set_text_interaction_flags(flags) }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct QMessageBox {
    pub(crate) inner: QBox<q::QMessageBox>,
}

impl_deref_binding!(QMessageBox, MessageBoxBinding);

impl Default for QMessageBox {
    fn default() -> Self {
        Self::new()
    }
}

impl QMessageBox {
    pub fn new() -> Self {
        let inner = unsafe { q::QMessageBox::new() };
        Self { inner }
    }
}
