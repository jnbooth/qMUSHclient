use std::fmt::{self, Debug, Formatter};
use std::ops::Deref;

use cpp_core::{CastFrom, CppDeletable, Ptr, StaticUpcast};
use qt_core::{QBox, QObject, QPtr};

pub enum QRef<T: CppDeletable + StaticUpcast<QObject>> {
    Borrowed(QPtr<T>),
    Owned(QBox<T>),
}

impl<T: StaticUpcast<QObject> + CppDeletable> Clone for QRef<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Borrowed(inner) => Self::Borrowed(inner.clone()),
            Self::Owned(inner) => Self::Borrowed(unsafe { QPtr::<T>::new(inner.as_ptr()) }),
        }
    }
}

impl<T: StaticUpcast<QObject> + CppDeletable> Debug for QRef<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Borrowed(inner) => inner.fmt(f),
            Self::Owned(inner) => inner.fmt(f),
        }
    }
}

impl<T: CppDeletable + StaticUpcast<QObject>> From<QPtr<T>> for QRef<T> {
    fn from(value: QPtr<T>) -> Self {
        Self::Borrowed(value)
    }
}

impl<T: CppDeletable + StaticUpcast<QObject>> From<QBox<T>> for QRef<T> {
    fn from(value: QBox<T>) -> Self {
        Self::Owned(value)
    }
}

impl<T: CppDeletable + StaticUpcast<QObject>> Deref for QRef<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Borrowed(inner) => inner.deref(),
            Self::Owned(inner) => inner.deref(),
        }
    }
}

impl<T: CppDeletable + StaticUpcast<QObject>> QRef<T> {
    /// Returns a reference to the value. Returns `None` if the pointer is null.
    ///
    /// ### Safety
    ///
    /// `self` must be valid.
    /// The content must not be read or modified through other ways while the returned reference
    /// exists.See type level documentation.
    pub unsafe fn as_raw_ref<'a>(&self) -> Option<&'a T> {
        unsafe {
            match self {
                Self::Borrowed(inner) => inner.as_raw_ref(),
                Self::Owned(inner) => inner.as_raw_ref(),
            }
        }
    }
}

impl<'a, T, U> CastFrom<&'a QRef<U>> for Ptr<T>
where
    U: StaticUpcast<T> + StaticUpcast<QObject> + CppDeletable,
{
    unsafe fn cast_from(value: &'a QRef<U>) -> Self {
        unsafe {
            match value {
                QRef::Borrowed(inner) => CastFrom::cast_from(inner),
                QRef::Owned(inner) => CastFrom::cast_from(inner),
            }
        }
    }
}
