use std::iter::{ExactSizeIterator, FusedIterator};
use std::ops::Deref;

use cpp_core::ops::{Begin, Decrement, End, Increment};
use cpp_core::vector_ops::Size;
use cpp_core::{CppBox, CppDeletable, Ref};

/// Represents Qt's `key()` function.
pub trait Key {
    /// Output type.
    type Output;

    /// Returns the current key of an iterator for a map-like object.
    ///
    /// # Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function
    /// may invoke arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn key(&self) -> Self::Output;
}

/// Represents Qt's `value()` function.
pub trait Value {
    /// Output type.
    type Output;

    /// Returns the current value of an iterator for a map-like object.
    ///
    /// # Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function
    /// may invoke arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn value(&self) -> Self::Output;
}

/// Represents Qt's `takeFirst()` function.
pub trait TakeFirst {
    /// Output type.
    type Output;

    /// Removes the first item in the list and returns it. Assumes the list is not empty.
    ///
    /// # Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function
    /// may invoke arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn take_first(&self) -> Self::Output;
}

/// Represents Qt's `takeLast()` function.
pub trait TakeLast {
    /// Output type.
    type Output;

    /// Removes the last item in the list and returns it. Assumes the list is not empty.
    ///
    /// # Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function
    /// may invoke arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn take_last(&self) -> Self::Output;
}

/// `Iterator` and `DoubleEndedIterator` backed by Qt's consuming iterators.
///
/// This object is produced by the blanket implementation of [`QIntoIterator`] on owned pointers of
/// list-like Qt types. You can also use the [`q_into_iter`] function to construct it manually from
/// an object without needing to import the `QIntoIterator` trait.
pub struct QIterator<T>
where
    T: CppDeletable,
{
    size: usize,
    list: CppBox<T>,
}

/// Constructs a Rust-style consuming iterator out of a list-like Qt object.
///
/// ### Safety
///
/// `size` must be valid. It's not possible to make any guarantees about safety, since `QIterator`
/// will call arbitrary C++ library code when used.
pub unsafe fn q_into_iter<T>(list: CppBox<T>) -> QIterator<T>
where
    T: CppDeletable + Size,
{
    QIterator {
        size: list.size(),
        list,
    }
}

impl<T> Iterator for QIterator<T>
where
    T: CppDeletable + TakeFirst,
{
    type Item = <T as TakeFirst>::Output;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            Some(unsafe { self.list.take_first() })
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.size, Some(self.size))
    }

    fn count(self) -> usize {
        self.size
    }
}

impl<T> FusedIterator for QIterator<T> where T: CppDeletable + TakeFirst {}

impl<T> ExactSizeIterator for QIterator<T>
where
    T: CppDeletable + TakeFirst,
{
    fn len(&self) -> usize {
        self.size
    }
}

impl<T> DoubleEndedIterator for QIterator<T>
where
    T: CppDeletable + TakeFirst + TakeLast<Output = <T as TakeFirst>::Output>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            None
        } else {
            self.size -= 1;
            Some(unsafe { self.list.take_last() })
        }
    }
}

/// Conversion into a `QIterator`. This trait is implemented automatically for any `CppBox` of a
/// list-like Qt type, meaning a type that implements [`Size`] and [`TakeFirst`]. If the type also
/// implements [`TakeLast`], then the `QIterator` produced implements `DoubleEndedIterator`.
pub trait QIntoIterator
where
    Self: Deref,
    Self::Target: CppDeletable,
{
    /// Constructs a Rust-style consuming iterator out of a list-like Qt object.
    ///
    /// ### Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function may invoke
    /// arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn into_iter(self) -> QIterator<Self::Target>;
}

impl<T> QIntoIterator for CppBox<T>
where
    T: CppDeletable + Size + TakeFirst,
{
    unsafe fn into_iter(self) -> QIterator<T> {
        q_into_iter(self)
    }
}

/// `Iterator` and `DoubleEndedIterator` for map-like Qt types.
///
/// This object is produced by the blanket implementation of [`QEntryIterable`] on map-like Qt
/// types. You can also use the [`q_entries`] function to construct it manually from an
/// object without needing to import the `QEntryIterable` trait.
pub struct QEntryIterator<T1, T2>
where
    T1: CppDeletable,
    T2: CppDeletable,
{
    begin: CppBox<T1>,
    end: CppBox<T2>,
}

/// Constructs a Rust-style iterator from a map-like Qt object that yields (key, value) entries.
///
/// ### Safety
///
/// `begin` and `end` must be valid. It's not possible to make any guarantees about safety, since
/// `QEntryIterator` will call arbitrary C++ library code when used.
pub unsafe fn q_entries<T1, T2>(begin: CppBox<T1>, end: CppBox<T2>) -> QEntryIterator<T1, T2>
where
    T1: CppDeletable,
    T2: CppDeletable,
{
    QEntryIterator { begin, end }
}

impl<T1, T2> Iterator for QEntryIterator<T1, T2>
where
    T1: CppDeletable + PartialEq<Ref<T2>> + Key + Value + Increment,
    T2: CppDeletable,
{
    type Item = (<T1 as Key>::Output, <T1 as Value>::Output);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.begin == self.end.as_ref() {
                None
            } else {
                let inner = &mut *self.begin.as_mut_raw_ptr();
                let value = (inner.key(), inner.value());
                let inner = &mut *self.begin.as_mut_raw_ptr();
                inner.inc();
                Some(value)
            }
        }
    }
}

impl<T1, T2> DoubleEndedIterator for QEntryIterator<T1, T2>
where
    T1: CppDeletable + PartialEq<Ref<T2>> + Key + Value + Increment,
    T2: CppDeletable
        + Decrement
        + Key<Output = <T1 as Key>::Output>
        + Value<Output = <T1 as Value>::Output>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.begin == self.end.as_ref() {
                None
            } else {
                let inner = &mut *self.end.as_mut_raw_ptr();
                inner.dec();
                let inner = &mut *self.end.as_mut_raw_ptr();
                let value = (inner.key(), inner.value());
                Some(value)
            }
        }
    }
}

/// Conversion into a `QEntryIterator`. This trait is implemented automatically for all map-like Qt
/// types, meaning types that produce STL iterator pairs with `Begin` and `End`, where
/// `Begin::Output` implements `Key`, `Value`, and `Increment`. If `End::Output` is the same as
/// `Begin::Output` and implements `Decrement`, then the `QEntryIterator` produced implements
/// `DoubleEndedIterator`.
pub trait QEntryIterable {
    type Iter: CppDeletable + Key + Value;

    /// Constructs a Rust-style iterator from a list-like Qt object that  yields (key, value)
    /// entries.
    ///
    /// ### Safety
    ///
    /// The caller must make sure `self` contains a valid pointer. This function may invoke
    /// arbitrary foreign code, so no safety guarantees can be made.
    unsafe fn entries(&self) -> QEntryIterator<Self::Iter, Self::Iter>;
}

impl<T, I> QEntryIterable for T
where
    T: Begin<Output = CppBox<I>> + End<Output = CppBox<I>>,
    I: CppDeletable + PartialEq<Ref<I>> + Key + Value + Increment,
{
    type Iter = I;

    unsafe fn entries(&self) -> QEntryIterator<I, I> {
        q_entries(self.begin(), self.end())
    }
}
