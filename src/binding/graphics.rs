use std::os::raw::{c_double, c_int};

use cpp_core::CppBox;
use qt_core::{QRect, QRectF};
use qt_gui::{QImage, QPainter};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rect<N> {
    pub left: N,
    pub top: N,
    pub width: N,
    pub height: N,
}

impl From<&QRect> for Rect<c_int> {
    fn from(value: &QRect) -> Self {
        let mut this = Self::default();
        unsafe {
            value.get_rect(
                &mut this.left,
                &mut this.top,
                &mut this.width,
                &mut this.height,
            );
        }
        this
    }
}

impl From<Rect<c_int>> for CppBox<QRect> {
    fn from(value: Rect<c_int>) -> Self {
        unsafe { QRect::from_4_int(value.left, value.top, value.width, value.height) }
    }
}

impl From<&QRectF> for Rect<c_double> {
    fn from(value: &QRectF) -> Self {
        let mut this = Self::default();
        unsafe {
            value.get_rect(
                &mut this.left,
                &mut this.top,
                &mut this.width,
                &mut this.height,
            );
        }
        this
    }
}

impl From<Rect<c_double>> for CppBox<QRectF> {
    fn from(value: Rect<c_double>) -> Self {
        unsafe { QRectF::from_4_double(value.left, value.top, value.width, value.height) }
    }
}

#[derive(Debug)]
pub struct RImage(pub(super) CppBox<QImage>);

impl From<CppBox<QImage>> for RImage {
    fn from(value: CppBox<QImage>) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Painter(pub(super) CppBox<QPainter>);
