use cpp_core::CppBox;
use qt_gui::QImage;

#[derive(Debug)]
pub struct RImage(pub(super) CppBox<QImage>);

impl From<CppBox<QImage>> for RImage {
    fn from(value: CppBox<QImage>) -> Self {
        Self(value)
    }
}
