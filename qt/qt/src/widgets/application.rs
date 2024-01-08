use std::os::raw::c_int;

use cpp_core::Ptr;
use qt_core::QCoreApplicationArgs;
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
    pub fn new() -> Self {
        let mut args = QCoreApplicationArgs::new();
        let (argc, argv) = args.get();
        let app = unsafe { q::QApplication::new_2a(argc, argv).as_ptr() };
        Self { inner: app }
    }

    pub fn exec() -> c_int {
        unsafe { q::QApplication::exec() }
    }

    pub fn init<F: FnOnce(QApplication) -> i32>(f: F) -> ! {
        q::QApplication::init(|inner| f(Self { inner }))
    }
}
