use cpp_core::CppBox;
use qt_gui::QImage;

use super::Binding;

#[derive(Debug, Binding)]
pub struct RImage(CppBox<QImage>);
