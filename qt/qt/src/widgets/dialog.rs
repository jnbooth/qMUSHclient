use std::ops::Deref;

use qt_core::QBox;
use qt_widgets as q;
use qt_widgets::q_dialog::DialogCode;

use super::WidgetBinding;

#[repr(transparent)]
#[derive(Debug)]
pub struct QDialog {
    pub(super) inner: QBox<q::QDialog>,
}

#[repr(transparent)]
pub struct DialogBinding {
    inner: q::QDialog,
}

impl DialogBinding {
    pub(super) fn cast(inner: &q::QDialog) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const q::QDialog as *const Self) }
    }

    /// Shows the dialog as a modal dialog, blocking until the user closes it.
    pub fn exec(&self) -> DialogCode {
        unsafe { self.inner.exec().into() }
    }
}

impl Deref for DialogBinding {
    type Target = WidgetBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}

impl Deref for QDialog {
    type Target = DialogBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
