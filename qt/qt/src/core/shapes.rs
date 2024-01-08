use std::os::raw::{c_double, c_int};

use cpp_core::CppBox;
use qt_core as q;

macro_rules! impl_qpoint {
    ($t:ty, $q:ty, $n:ty) => {
        impl $t {
            pub const fn x(&self) -> $n {
                self.x
            }

            pub const fn y(&self) -> $n {
                self.y
            }
        }

        impl From<&$q> for $t {
            fn from(value: &$q) -> Self {
                unsafe {
                    Self {
                        x: value.x(),
                        y: value.y(),
                    }
                }
            }
        }

        impl From<$t> for CppBox<$q> {
            fn from(value: $t) -> Self {
                unsafe { <$q>::new_2a(value.x, value.y) }
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QPoint {
    pub x: c_int,
    pub y: c_int,
}
impl_qpoint!(QPoint, q::QPoint, c_int);

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QPointF {
    pub x: c_double,
    pub y: c_double,
}
impl_qpoint!(QPointF, q::QPointF, c_double);

macro_rules! impl_qrect {
    ($t:ty, $q:ty, $n:ty, $from:ident) => {
        impl $t {
            pub const fn width(&self) -> $n {
                self.width
            }
            pub const fn height(&self) -> $n {
                self.height
            }
            pub const fn left(&self) -> $n {
                self.left
            }
            pub const fn top(&self) -> $n {
                self.top
            }
        }

        impl From<&$q> for $t {
            fn from(value: &$q) -> Self {
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

        impl From<$t> for CppBox<$q> {
            fn from(value: $t) -> Self {
                unsafe { <$q>::$from(value.left, value.top, value.width, value.height) }
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QRect {
    pub left: c_int,
    pub top: c_int,
    pub width: c_int,
    pub height: c_int,
}
impl_qrect!(QRect, q::QRect, c_int, from_4_int);

impl QRect {
    pub const fn right(&self) -> c_int {
        self.left + self.width
    }
    pub const fn bottom(&self) -> c_int {
        self.top + self.height
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QRectF {
    pub left: c_double,
    pub top: c_double,
    pub width: c_double,
    pub height: c_double,
}
impl_qrect!(QRectF, q::QRectF, c_double, from_4_double);

impl QRectF {
    pub fn right(&self) -> c_double {
        self.left + self.width
    }
    pub fn bottom(&self) -> c_double {
        self.top + self.height
    }
}

macro_rules! impl_qsize {
    ($t:ty, $q:ty, $n:ty) => {
        impl $t {
            pub const fn width(&self) -> $n {
                self.width
            }

            pub const fn height(&self) -> $n {
                self.height
            }
        }

        impl From<&$q> for $t {
            fn from(value: &$q) -> Self {
                unsafe {
                    Self {
                        width: value.width(),
                        height: value.height(),
                    }
                }
            }
        }

        impl From<$t> for CppBox<$q> {
            fn from(value: $t) -> Self {
                unsafe { <$q>::new_2a(value.width, value.height) }
            }
        }
    };
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct QSize {
    pub width: c_int,
    pub height: c_int,
}
impl_qsize!(QSize, q::QSize, c_int);

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QSizeF {
    pub width: c_double,
    pub height: c_double,
}
impl_qsize!(QSizeF, q::QSizeF, c_double);
