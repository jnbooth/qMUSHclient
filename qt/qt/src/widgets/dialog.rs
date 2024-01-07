use qt_core::QBox;
use qt_widgets as q;
use qt_widgets::q_dialog::DialogCode;

qt_binding!(DialogBinding, q::QDialog, super::WidgetBinding);

impl DialogBinding {
    /// Shows the dialog as a modal dialog, blocking until the user closes it.
    pub fn exec(&self) -> DialogCode {
        unsafe { self.inner.exec().into() }
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct QDialog {
    pub(crate) inner: QBox<q::QDialog>,
}

impl_deref_binding!(QDialog, DialogBinding);
