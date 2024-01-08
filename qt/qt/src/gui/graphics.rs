use cpp_core::CppBox;
use qt_gui as q;

use crate::core::QVariant;

#[derive(Debug)]
#[repr(transparent)]
pub struct QImage {
    pub(crate) inner: CppBox<q::QImage>,
}

impl_eq_cpp!(QImage);

impl From<CppBox<q::QImage>> for QImage {
    fn from(value: CppBox<q::QImage>) -> Self {
        Self { inner: value }
    }
}

impl From<QImage> for QVariant {
    fn from(value: QImage) -> Self {
        QVariant::from(value.inner)
    }
}

impl From<&QImage> for QVariant {
    fn from(value: &QImage) -> Self {
        QVariant::from(&value.inner)
    }
}

impl QImage {
    pub fn new() -> Self {
        Self {
            inner: unsafe { q::QImage::new() },
        }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct QPainter {
    pub(crate) inner: CppBox<q::QPainter>,
}
