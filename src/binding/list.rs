use std::ops::Deref;
use std::os::raw::c_int;

use cpp_core::{cpp_iter, CastInto, CppBox, CppDeletable, CppIterator, Ref};
use qt_core::*;
use qt_widgets::*;

type CppMap<I, From, To> = std::iter::Map<CppIterator<I, I>, fn(From) -> To>;

pub trait QList: CppDeletable {
    type Item;
    type Iter: Iterator<Item = Self::Item>;
    fn new() -> CppBox<Self>;
    fn reserve(&self, size: c_int);
    /// # Safety
    ///
    /// `item` must be valid.
    unsafe fn push(&self, item: Self::Item);
    /// # Safety
    ///
    /// `other` must be valid.
    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R);
    /// # Safety
    ///
    /// `item` must be valid.
    unsafe fn repeat(&self, item: Self::Item, size: c_int) -> CppBox<Self>
    where
        Self::Item: Copy,
    {
        let this = Self::new();
        this.reserve(size);
        unsafe {
            for _ in 0..size {
                this.push(item);
            }
        }
        this
    }
    /// # Safety
    ///
    /// `items` must be valid.
    unsafe fn from_array<const N: usize>(items: [Self::Item; N]) -> CppBox<Self>
    where
        Self::Item: Copy,
    {
        let this = Self::new();
        this.reserve(N as c_int);
        unsafe {
            for item in &items {
                this.push(*item);
            }
        }
        this
    }
    /// # Safety
    ///
    /// All items produced by `iter` must be valid
    unsafe fn from_iter<I: Iterator<Item = Self::Item>>(iter: I) -> CppBox<Self> {
        let this = Self::new();
        this.reserve(iter.size_hint().0 as c_int);
        unsafe {
            for item in iter {
                this.push(item);
            }
        }
        this
    }

    /// # Safety
    ///
    /// See [Qt Documentation: STL-Style Iterators](https://doc.qt.io/qt-5/containers.html#stl-style-iterators).
    unsafe fn cpp_iter(&self) -> Self::Iter;
}

impl QList for QStringList {
    type Item = CppBox<QString>;
    type Iter = CppMap<qt_core::q_list_of_q_string::ConstIterator, Ref<QString>, Self::Item>;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
    }
    unsafe fn cpp_iter(&self) -> Self::Iter {
        unsafe { cpp_iter(self.const_begin(), self.const_end()).map(|x| QString::new_copy(x)) }
    }
    unsafe fn push(&self, item: Self::Item) {
        unsafe {
            self.append_q_string(&item);
        }
    }
    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
        unsafe {
            self.append_q_list_of_q_string(other.cast_into());
        }
    }
}

impl QList for QListOfQString {
    type Item = CppBox<QString>;
    type Iter = CppMap<qt_core::q_list_of_q_string::ConstIterator, Ref<QString>, Self::Item>;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
    }
    unsafe fn cpp_iter(&self) -> Self::Iter {
        unsafe { cpp_iter(self.const_begin(), self.const_end()).map(|x| QString::new_copy(x)) }
    }
    unsafe fn push(&self, item: Self::Item) {
        unsafe {
            self.append_q_string(&item);
        }
    }

    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
        unsafe {
            self.append_q_list_of_q_string(other.cast_into());
        }
    }
}

impl QList for QListOfInt {
    type Item = c_int;
    type Iter = CppMap<qt_core::q_list_of_int::ConstIterator, *const c_int, Self::Item>;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
    }
    unsafe fn cpp_iter(&self) -> Self::Iter {
        unsafe { cpp_iter(self.const_begin(), self.const_end()).map(|x| *x) }
    }
    unsafe fn push(&self, item: Self::Item) {
        unsafe {
            self.append_int(&item);
        }
    }

    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
        unsafe {
            self.append_q_list_of_int(other);
        }
    }
}

impl QList for QListOfQVariant {
    type Item = CppBox<QVariant>;
    type Iter = CppMap<qt_core::q_list_of_q_variant::ConstIterator, Ref<QVariant>, Self::Item>;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
    }
    unsafe fn cpp_iter(&self) -> Self::Iter {
        unsafe { cpp_iter(self.const_begin(), self.const_end()).map(|x| QVariant::new_copy(x)) }
    }
    unsafe fn push(&self, item: Self::Item) {
        unsafe {
            self.append_q_variant(&item);
        }
    }

    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
        unsafe {
            self.append_q_list_of_q_variant(other);
        }
    }
}

impl QList for QListOfQTreeWidgetItem {
    type Item = *const *mut QTreeWidgetItem;
    type Iter = CppIterator<
        qt_widgets::q_list_of_q_tree_widget_item::ConstIterator,
        qt_widgets::q_list_of_q_tree_widget_item::ConstIterator,
    >;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
    }
    unsafe fn cpp_iter(&self) -> Self::Iter {
        unsafe { cpp_iter(self.const_begin(), self.const_end()) }
    }
    unsafe fn push(&self, item: Self::Item) {
        unsafe {
            self.append_q_tree_widget_item(item.as_ref().unwrap());
        }
    }

    unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
        unsafe {
            self.append_q_list_of_q_tree_widget_item(other);
        }
    }
}
