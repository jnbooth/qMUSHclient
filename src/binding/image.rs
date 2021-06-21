use super::Binding;
use cpp_core::CppBox;
use qt_gui::QImage;

#[derive(Debug, Binding)]
pub struct RImage(CppBox<QImage>);
