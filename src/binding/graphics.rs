use std::ops::Add;
use std::os::raw::{c_double, c_int};

use cpp_core::CppBox;
use qt_core::{QPoint, QRect, QRectF};
use qt_gui::{QImage, QPainter};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RPoint {
    x: c_int,
    y: c_int,
}

impl From<&QPoint> for RPoint {
    fn from(value: &QPoint) -> Self {
        unsafe {
            Self {
                x: value.x(),
                y: value.y(),
            }
        }
    }
}

impl From<RPoint> for CppBox<QPoint> {
    fn from(value: RPoint) -> Self {
        unsafe { QPoint::new_2a(value.x, value.y) }
    }
}

impl RPoint {
    pub fn new(x: c_int, y: c_int) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> c_int {
        self.x
    }

    pub fn y(&self) -> c_int {
        self.y
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RRect<N> {
    left: N,
    top: N,
    width: N,
    height: N,
}

impl<N: Copy + Add<N, Output = N>> RRect<N> {
    pub fn width(&self) -> N {
        self.width
    }
    pub fn height(&self) -> N {
        self.height
    }
    pub fn left(&self) -> N {
        self.left
    }
    pub fn top(&self) -> N {
        self.top
    }
    pub fn right(&self) -> N {
        self.left + self.width
    }
    pub fn bottom(&self) -> N {
        self.top + self.height
    }
}

impl From<&QRect> for RRect<c_int> {
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

impl From<RRect<c_int>> for CppBox<QRect> {
    fn from(value: RRect<c_int>) -> Self {
        unsafe { QRect::from_4_int(value.left, value.top, value.width, value.height) }
    }
}

impl From<&QRectF> for RRect<c_double> {
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

impl From<RRect<c_double>> for CppBox<QRectF> {
    fn from(value: RRect<c_double>) -> Self {
        unsafe { QRectF::from_4_double(value.left, value.top, value.width, value.height) }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct RImage {
    pub(super) inner: CppBox<QImage>,
}

impl_eq_cpp!(RImage);

impl From<CppBox<QImage>> for RImage {
    fn from(value: CppBox<QImage>) -> Self {
        Self { inner: value }
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Painter {
    pub(super) inner: CppBox<QPainter>,
}
