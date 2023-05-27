use std::ops::Deref;

use qt_core::QBox;
use qt_widgets::q_dialog::DialogCode;
use qt_widgets::QDialog;

use super::WidgetBinding;

#[repr(transparent)]
#[derive(Debug)]
pub struct RDialog {
    pub(super) inner: QBox<QDialog>,
}

#[repr(transparent)]
pub struct DialogBinding {
    inner: QDialog,
}

impl DialogBinding {
    pub(super) fn cast(inner: &QDialog) -> &Self {
        // SAFETY: #[repr(transparent)]
        unsafe { &*(inner as *const QDialog as *const Self) }
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

impl Deref for RDialog {
    type Target = DialogBinding;

    fn deref(&self) -> &Self::Target {
        Self::Target::cast(&self.inner)
    }
}
