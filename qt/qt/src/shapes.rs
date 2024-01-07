use std::ops::Add;
use std::os::raw::{c_double, c_int};

use cpp_core::CppBox;
use qt_core as q;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QPoint {
    x: c_int,
    y: c_int,
}

impl From<&q::QPoint> for QPoint {
    fn from(value: &q::QPoint) -> Self {
        unsafe {
            Self {
                x: value.x(),
                y: value.y(),
            }
        }
    }
}

impl From<QPoint> for CppBox<q::QPoint> {
    fn from(value: QPoint) -> Self {
        unsafe { q::QPoint::new_2a(value.x, value.y) }
    }
}

impl QPoint {
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
pub struct Rect<N> {
    left: N,
    top: N,
    width: N,
    height: N,
}

impl<N: Copy + Add<N, Output = N>> Rect<N> {
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

pub type QRect = Rect<c_int>;

impl From<&q::QRect> for QRect {
    fn from(value: &q::QRect) -> Self {
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

impl From<QRect> for CppBox<q::QRect> {
    fn from(value: QRect) -> Self {
        unsafe { q::QRect::from_4_int(value.left, value.top, value.width, value.height) }
    }
}

pub type QRectF = Rect<c_double>;

impl From<&q::QRectF> for QRectF {
    fn from(value: &q::QRectF) -> Self {
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

impl From<QRectF> for CppBox<q::QRectF> {
    fn from(value: QRectF) -> Self {
        unsafe { q::QRectF::from_4_double(value.left, value.top, value.width, value.height) }
    }
}
