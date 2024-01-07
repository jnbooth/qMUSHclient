use std::ops::Deref;
use std::os::raw::c_int;

use cpp_core::{CastInto, CppBox, CppDeletable, Ref};
use qt_core::*;
use qt_widgets::*;

pub trait QList: CppDeletable {
    type Item;

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
    unsafe fn from_iter<I: IntoIterator<Item = Self::Item>>(iter: I) -> CppBox<Self> {
        let iter = iter.into_iter();
        let this = Self::new();
        this.reserve(iter.size_hint().0 as c_int);
        unsafe {
            for item in iter {
                this.push(item);
            }
        }
        this
    }
}

impl QList for QStringList {
    type Item = CppBox<QString>;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.deref().reserve(size);
        }
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

macro_rules! impl_qlist {
    ($t:ty, $item:ty, $append_one:ident, $append_multi:ident) => {
        impl QList for $t {
            type Item = $item;

            fn new() -> CppBox<Self> {
                unsafe { Self::new() }
            }
            fn reserve(&self, size: c_int) {
                unsafe {
                    self.reserve(size);
                }
            }
            unsafe fn push(&self, item: Self::Item) {
                unsafe {
                    self.$append_one(&item);
                }
            }
            unsafe fn append<R: CastInto<Ref<Self>>>(&self, other: R) {
                unsafe {
                    self.$append_multi(other.cast_into());
                }
            }
        }
    };
}

impl_qlist!(
    QListOfQString,
    CppBox<QString>,
    append_q_string,
    append_q_list_of_q_string
);
impl_qlist!(QListOfInt, c_int, append_int, append_q_list_of_int);
impl_qlist!(
    QListOfQVariant,
    CppBox<QVariant>,
    append_q_variant,
    append_q_list_of_q_variant
);

impl QList for QListOfQTreeWidgetItem {
    type Item = *const *mut QTreeWidgetItem;

    fn new() -> CppBox<Self> {
        unsafe { Self::new() }
    }
    fn reserve(&self, size: c_int) {
        unsafe {
            self.reserve(size);
        }
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
