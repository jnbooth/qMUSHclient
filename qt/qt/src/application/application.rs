use std::os::raw::c_int;
use std::process;

use qt_core::{QBox, QCoreApplicationArgs};
use qt_widgets as q;

use crate::object::ObjectBinding;

qt_binding!(ApplicationBinding, qt_widgets::QApplication, ObjectBinding);

#[repr(transparent)]
pub struct QApplication {
    pub(crate) inner: QBox<q::QApplication>,
}

impl_deref_binding!(QApplication, ApplicationBinding);

impl QApplication {
    pub fn new() -> Self {
        let mut args = QCoreApplicationArgs::new();
        let (argc, argv) = args.get();
        let app = unsafe { q::QApplication::new_2a(argc, argv) };
        Self { inner: app }
    }

    pub fn exec() -> c_int {
        unsafe { q::QApplication::exec() }
    }

    pub fn init<F: FnOnce(&QApplication) -> i32>(f: F) -> ! {
        let exit_code = {
            let app = QApplication::new();
            f(&app)
        }; // drop `app` and `args`
        process::exit(exit_code)
    }
}
