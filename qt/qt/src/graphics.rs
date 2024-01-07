use cpp_core::CppBox;
use qt_gui as q;

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

#[derive(Debug)]
#[repr(transparent)]
pub struct Painter {
    pub(crate) inner: CppBox<q::QPainter>,
}
