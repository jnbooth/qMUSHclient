use cpp_core::Ptr;
use qt_core::QPtr;
use qt_widgets::QWidget;

use super::super::traits::Widget;
use crate::binding::font::RFontMetrics;
use crate::binding::RRect;

#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct RWidget {
    pub(super) inner: QPtr<QWidget>,
}

impl Widget for RWidget {
    fn widget(&self) -> Ptr<QWidget> {
        unsafe { self.inner.as_ptr() }
    }
}

impl RWidget {
    /// # Safety
    ///
    /// `inner` must be valid and non-null.
    pub unsafe fn new(inner: QPtr<QWidget>) -> Self {
        Self { inner }
    }

    pub fn font_metrics(&self) -> RFontMetrics {
        RFontMetrics::new(unsafe { self.inner.font_metrics() })
    }

    pub fn height(&self) -> i32 {
        unsafe { self.inner.height() }
    }

    pub fn rect(&self) -> RRect<i32> {
        RRect::from(unsafe { &*self.inner.rect() })
    }

    pub fn width(&self) -> i32 {
        unsafe { self.inner.width() }
    }
}
