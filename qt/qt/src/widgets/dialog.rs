use cpp_core::{CastFrom, Ptr};
use qt_core::QBox;
use qt_widgets as q;
use qt_widgets::q_dialog::DialogCode;

use crate::traits::Widget;

qt_binding!(DialogBinding, q::QDialog, super::widget::WidgetBinding);

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

impl Widget for QDialog {
    fn widget(&self) -> Ptr<q::QWidget> {
        // SAFETY: self.inner is valid
        unsafe { CastFrom::cast_from(&self.inner) }
    }
}
