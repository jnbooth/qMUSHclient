use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_widgets as q;

qt_binding!(
    ApplicationBinding,
    qt_widgets::QApplication,
    crate::gui::GuiApplicationBinding
);

#[repr(transparent)]
pub struct QApplication {
    pub(crate) inner: Ptr<q::QApplication>,
}

impl_deref_binding!(QApplication, ApplicationBinding);

impl QApplication {
    pub fn exec() -> c_int {
        unsafe { q::QApplication::exec() }
    }

    pub fn init<F: FnOnce(QApplication) -> i32>(f: F) -> ! {
        q::QApplication::init(|inner| f(Self { inner }))
    }
}
