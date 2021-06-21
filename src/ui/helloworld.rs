use std::rc::Rc;

use cpp_core::{CastInto, Ptr};
use qt_widgets::QWidget;

use crate::ui::uic;

pub struct HelloWorld {
    ui: uic::HelloWorld,
}

impl HelloWorld {
    fn new<P: CastInto<Ptr<QWidget>>>(parent: P) -> Rc<Self> {
        let this = Rc::new(Self {
            ui: uic::HelloWorld::load(parent),
        });
        unsafe { this.init() };
        this
    }

    unsafe fn init(self: &Rc<Self>) {
        /* add slot + signal connectors, etc. */
    }
}
