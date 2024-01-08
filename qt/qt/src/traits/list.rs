use std::os::raw::{c_double, c_int};

use cpp_core::{CppBox, CppDeletable};
use qt_core::*;
use qt_gui::QListOfDouble;
use qt_widgets::*;

use super::Printable;

pub trait QList<T>: CppDeletable {
    /// # Safety
    ///
    /// All items produced by `iter` must be valid
    unsafe fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> CppBox<Self>;
}

macro_rules! impl_qlist {
    ($item:ty, $append:ident) => {
        unsafe fn from_iter<I: IntoIterator<Item = $item>>(iter: I) -> CppBox<Self> {
            let it = iter.into_iter();
            unsafe {
                let this = Self::new();
                this.reserve(it.size_hint().0 as c_int);
                for item in it {
                    this.$append(item);
                }
                this
            }
        }
    };
}

macro_rules! impl_qlist_borrow {
    ($item:ty, $append:ident) => {
        unsafe fn from_iter<I: IntoIterator<Item = $item>>(iter: I) -> CppBox<Self> {
            let it = iter.into_iter();
            unsafe {
                let this = Self::new();
                this.reserve(it.size_hint().0 as c_int);
                for item in it {
                    this.$append(&item);
                }
                this
            }
        }
    };
}

macro_rules! impl_qlist_printable {
    ($t:ty, $append:ident) => {
        unsafe fn from_iter<I: IntoIterator<Item = $t>>(iter: I) -> CppBox<Self> {
            let it = iter.into_iter();
            unsafe {
                let this = Self::new();
                this.reserve(it.size_hint().0 as c_int);
                for item in it {
                    this.$append(&item.to_print());
                }
                this
            }
        }
    };
}

impl QList<c_double> for QListOfDouble {
    impl_qlist_borrow!(c_double, append_double);
}

impl QList<c_int> for QListOfInt {
    impl_qlist_borrow!(c_int, append_int);
}

impl<S: Printable> QList<S> for QListOfQString {
    impl_qlist_printable!(S, append_q_string);
}

impl QList<*const *mut QTreeWidgetItem> for QListOfQTreeWidgetItem {
    impl_qlist!(*const *mut QTreeWidgetItem, append_q_tree_widget_item);
}

impl QList<CppBox<QVariant>> for QListOfQVariant {
    impl_qlist_borrow!(CppBox<QVariant>, append_q_variant);
}
impl<'a> QList<&'a CppBox<QVariant>> for QListOfQVariant {
    impl_qlist!(&'a CppBox<QVariant>, append_q_variant);
}

impl<S: Printable> QList<S> for QStringList {
    impl_qlist_printable!(S, append_q_string);
}
