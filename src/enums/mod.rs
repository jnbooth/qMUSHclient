use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{ExactSizeIterator, FusedIterator, Iterator};
use std::ops::{Bound, RangeBounds};

#[macro_use]
mod set;
pub use set::{EnumSet, Wordlike};
mod map;
pub use map::EnumMap;

pub trait Enum: Copy + Ord {
    /// Bitwise representation of the type.
    type Rep;

    /// Total number of values in the type.
    const SIZE: usize;

    /// Smallest value in the type.
    ///
    /// Rules: for all `x`, `x.succ() != Some(Self::MIN)`.
    const MIN: Self;

    /// Largest value in the type.
    ///
    /// Rule: for all `x`, `x.pred() != Some(Self::MAX)`.
    const MAX: Self;

    /// Returns `self`'s successor, or `None` if `self == Self::MAX`.
    ///
    /// Rule: for all `x`, `(x == Self::MAX) == x.succ().is_none()`.
    fn succ(self) -> Option<Self>;

    /// Returns `self`'s predecessor, or `None` if `self == Self::MIN`.
    ///
    /// Rule: for all `x`, `(x == Self::MIN) == x.pred().is_none()`.
    fn pred(self) -> Option<Self>;

    /// Bitwise representation of the value.
    fn bit(self) -> Self::Rep;

    /// The value's position in a complete enumeration of the type.
    fn index(self) -> usize;

    fn enumerate<R: RangeBounds<Self>>(range: R) -> Enumeration<Self> {
        fn invalid_enum<E: Enum>() -> Enumeration<E> {
            Enumeration {
                start: E::MIN,
                end: E::MIN,
                finished: true,
            }
        }
        let start = match range.start_bound() {
            Bound::Unbounded => Self::MIN,
            Bound::Included(&t) => t,
            Bound::Excluded(&t) => match t.succ() {
                Some(succ) => succ,
                None => return invalid_enum(),
            },
        };
        let end = match range.end_bound() {
            Bound::Unbounded => Self::MAX,
            Bound::Included(&t) => t,
            Bound::Excluded(&t) => match t.pred() {
                Some(pred) => pred,
                None => return invalid_enum(),
            },
        };
        if start.index() > end.index() {
            return invalid_enum();
        }
        Enumeration {
            start,
            end,
            finished: false,
        }
    }
}

impl Enum for bool {
    type Rep = u8;
    const SIZE: usize = 2;
    const MIN: Self = false;
    const MAX: Self = true;
    fn succ(self) -> Option<Self> {
        if self {
            None
        } else {
            Some(true)
        }
    }
    fn pred(self) -> Option<Self> {
        if self {
            Some(false)
        } else {
            None
        }
    }
    fn bit(self) -> Self::Rep {
        1 << (self as u8)
    }
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enumeration<T> {
    finished: bool,
    start: T,
    end: T,
}

impl<T: Enum> Iterator for Enumeration<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else if self.start == self.end {
            self.finished = true;
            Some(self.start)
        } else {
            let at = self.start;
            self.start = at
                .succ()
                .expect("got None from calling Enum::succ() where < Enum::MAX");
            Some(at)
        }
    }

    fn count(self) -> usize {
        self.end.index() - self.start.index()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.count();
        (exact, Some(exact))
    }
}
impl<T: Enum> DoubleEndedIterator for Enumeration<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else if self.start == self.end {
            Some(self.end)
        } else {
            let at = self.end;
            self.end = at
                .pred()
                .expect("got None from calling Enum::pred() where > Enum::MIN");
            Some(at)
        }
    }
}
impl<T: Enum> FusedIterator for Enumeration<T> {}
impl<T: Enum> ExactSizeIterator for Enumeration<T> {
    fn len(&self) -> usize {
        self.count()
    }
}
