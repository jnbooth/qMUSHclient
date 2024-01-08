#![warn(unsafe_op_in_unsafe_fn)]
#[macro_use]
extern crate enumeration;
#[macro_use]
extern crate enumeration_derive;

macro_rules! impl_deref_binding {
    ($t:ty, $inner:ty) => {
        impl std::ops::Deref for $t {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                Self::Target::cast(&self.inner)
            }
        }
    };
}

macro_rules! impl_eq_cpp {
    ($t:ty) => {
        impl PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                unsafe { self.inner.eq(&other.inner.as_ref()) }
            }
        }

        impl Eq for $t {}
    };
}

macro_rules! qt_binding {
    ($t:ident, $inner:ty) => {
        #[repr(transparent)]
        pub struct $t {
            #[allow(dead_code)]
            pub(crate) inner: $inner,
        }

        impl $t {
            pub(crate) fn cast(inner: &$inner) -> &Self {
                // SAFETY: #[repr(transparent)]
                unsafe { &*(inner as *const $inner as *const Self) }
            }
        }
    };

    ($t:ident, $inner:ty, $inherit:ty) => {
        qt_binding!($t, $inner);

        impl std::ops::Deref for $t {
            type Target = $inherit;

            fn deref(&self) -> &Self::Target {
                Self::Target::cast(&self.inner)
            }
        }
    };
}

pub mod core;

pub mod gui;

pub mod io;
pub use io::QIODevice;

pub mod traits;

pub mod widgets;
