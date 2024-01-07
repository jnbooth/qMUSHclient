use std::os::raw::c_int;
use std::time::Duration;

use enumeration::Enum;
use qt_core as q;
use qt_core::{QBox, SlotNoArgs};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum TimerKind {
    Once,
    Repeating,
}

#[derive(Debug)]
pub struct QTimer {
    inner: QBox<q::QTimer>,
}

impl Default for QTimer {
    fn default() -> Self {
        Self {
            inner: unsafe { q::QTimer::new_1a(q::QAbstractEventDispatcher::instance_0a()) },
        }
    }
}

impl QTimer {
    pub fn new(kind: TimerKind, interval: Duration) -> Self {
        unsafe {
            let timer = q::QTimer::new_1a(q::QAbstractEventDispatcher::instance_0a());
            timer.set_interval(interval.subsec_millis() as c_int);
            if kind == TimerKind::Once {
                timer.set_single_shot(true);
            }
            Self { inner: timer }
        }
    }

    pub fn kind(&self) -> TimerKind {
        if unsafe { self.inner.is_single_shot() } {
            TimerKind::Once
        } else {
            TimerKind::Repeating
        }
    }

    pub fn connect<F: FnMut() + 'static>(&self, f: F) {
        unsafe {
            self.inner
                .timeout()
                .connect(&SlotNoArgs::new(&self.inner, f));
        }
    }

    pub fn disconnect_all(&self) {
        unsafe {
            self.inner.disconnect();
        }
    }

    pub fn start(&self) {
        unsafe { self.inner.start_0a() }
    }
}
