use std::borrow::{Borrow, ToOwned};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};

use hashbrown::hash_map::DefaultHashBuilder;

use super::AsRefHashMap;

pub trait ToCaseFold<Target> {
    fn to_case_fold(self) -> Target;
}

pub enum CaseFold<'a, S: 'a + ToOwned + ?Sized> {
    Ascii(<S as ToOwned>::Owned),
    BorrowedAscii(&'a S),
    Unicode(<S as ToOwned>::Owned),
    BorrowedUnicode(&'a S),
}

impl<'a, S: 'a + ToOwned + ?Sized> Clone for CaseFold<'a, S> {
    fn clone(&self) -> Self {
        match self {
            Self::BorrowedAscii(b) => Self::BorrowedAscii(b),
            Self::Ascii(o) => {
                let b: &S = o.borrow();
                Self::Ascii(b.to_owned())
            }
            Self::BorrowedUnicode(b) => Self::BorrowedUnicode(b),
            Self::Unicode(o) => {
                let b: &S = o.borrow();
                Self::Unicode(b.to_owned())
            }
        }
    }
}

impl<'a, S: ToOwned + AsRef<str> + ?Sized> CaseFold<'a, S> {
    pub fn new(s: S::Owned) -> Self {
        if s.borrow().as_ref().is_ascii() {
            Self::Ascii(s)
        } else {
            Self::Unicode(s)
        }
    }
    pub fn borrowed(s: &'a S) -> Self {
        if s.as_ref().is_ascii() {
            Self::BorrowedAscii(s)
        } else {
            Self::BorrowedUnicode(s)
        }
    }
}

impl<'a, S: ?Sized + ToOwned + AsRef<str>> CaseFold<'a, S> {
    #[inline]
    fn as_str(&self) -> &str {
        match self {
            Self::Ascii(s) => s.borrow(),
            Self::BorrowedAscii(s) => s,
            Self::Unicode(s) => s.borrow(),
            Self::BorrowedUnicode(s) => s,
        }
        .as_ref()
    }

    // to_case_fold is a simple cast here (see below). Should be a no-op as long as everything is
    // inlined. Because as_str calls as_ref, as_ref calls in ascii::CaseFold and unicode::CaseFold
    // will be no-ops as well. Therefore, this approach wins out over holding actual
    // ascii::CaseFolds or unicode::CaseFolds in the enum fields.
    #[inline]
    fn as_unicode(&self) -> &unicode::CaseFold<str> {
        self.as_str().to_case_fold()
    }

    #[inline]
    fn try_ascii(&self) -> Result<&ascii::CaseFold<str>, &unicode::CaseFold<str>> {
        match self {
            Self::Ascii(s) => Ok(s.borrow().as_ref().to_case_fold()),
            Self::BorrowedAscii(s) => Ok(s.as_ref().to_case_fold()),
            Self::Unicode(s) => Err(s.borrow().as_ref().to_case_fold()),
            Self::BorrowedUnicode(s) => Err(s.as_ref().to_case_fold()),
        }
    }

    #[inline]
    #[allow(clippy::type_complexity)]
    fn pair<'b, Rhs: ?Sized + ToOwned + AsRef<str>>(
        &'a self,
        other: &'b CaseFold<'b, Rhs>,
    ) -> Result<
        (&'a ascii::CaseFold<str>, &'b ascii::CaseFold<str>),
        (&'a unicode::CaseFold<str>, &'b unicode::CaseFold<str>),
    > {
        match (self.try_ascii(), other.try_ascii()) {
            (Ok(x), Ok(y)) => Ok((x, y)),
            (Ok(..), Err(y)) => Err((self.as_unicode(), y)),
            (Err(x), Ok(..)) => Err((x, other.as_unicode())),
            (Err(x), Err(y)) => Err((x, y)),
        }
    }
}

impl<'a, 'b, S, Rhs> PartialEq<CaseFold<'a, Rhs>> for CaseFold<'b, S>
where
    S: ?Sized + ToOwned + AsRef<str>,
    Rhs: ?Sized + ToOwned + AsRef<str>,
{
    #[inline]
    fn eq(&self, other: &CaseFold<Rhs>) -> bool {
        match self.pair(other) {
            Ok((x, y)) => x == y,
            Err((x, y)) => x == y,
        }
    }
}

impl<'a, S: ?Sized + AsRef<str> + ToOwned> Eq for CaseFold<'a, S> {}

impl<'a, S: ?Sized + AsRef<str> + ToOwned> Hash for CaseFold<'a, S> {
    #[inline]
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        match self.try_ascii() {
            Ok(x) => x.hash(hasher),
            Err(x) => x.hash(hasher),
        }
    }
}

impl<'a, 'b, S, Rhs> PartialOrd<CaseFold<'a, Rhs>> for CaseFold<'b, S>
where
    S: ?Sized + AsRef<str> + ToOwned,
    Rhs: ?Sized + AsRef<str> + ToOwned,
{
    #[inline]
    fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<Ordering> {
        match self.pair(other) {
            Ok((x, y)) => x.partial_cmp(y),
            Err((x, y)) => x.partial_cmp(y),
        }
    }
}

impl<'a, S: ?Sized + AsRef<str> + ToOwned> Ord for CaseFold<'a, S> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        match self.pair(other) {
            Ok((x, y)) => x.cmp(y),
            Err((x, y)) => x.cmp(y),
        }
    }
}

impl<'a, S: ?Sized + AsRef<str> + ToOwned> Debug for CaseFold<'a, S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self.as_str(), f)
    }
}

impl<'a, S: ?Sized + AsRef<str> + ToOwned> Display for CaseFold<'a, S> {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(self.as_str(), f)
    }
}

impl<'a, S: AsRef<str> + ToOwned> Borrow<unicode::CaseFold<str>> for CaseFold<'a, S> {
    fn borrow(&self) -> &unicode::CaseFold<str> {
        self.as_unicode()
    }
}

pub type CaseFoldMap<K, V, S = DefaultHashBuilder> =
    AsRefHashMap<unicode::CaseFold<str>, CaseFold<'static, K>, V, S>;

macro_rules! impl_ci {
    ($t:ty) => {
        impl<S: ?Sized + AsRef<$t>> Eq for CaseFold<S> {}

        impl<S, Rhs> PartialOrd<CaseFold<Rhs>> for CaseFold<S>
        where
            S: ?Sized + AsRef<$t>,
            Rhs: ?Sized + AsRef<$t>,
        {
            #[inline]
            fn partial_cmp(&self, other: &CaseFold<Rhs>) -> Option<std::cmp::Ordering> {
                Some(self.caseless_iter().cmp(other.caseless_iter()))
            }
        }

        impl<S: ?Sized + AsRef<$t>> Ord for CaseFold<S> {
            #[inline]
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.caseless_iter().cmp(other.caseless_iter())
            }
        }

        impl<S> CaseFold<S> {
            pub const fn new(s: S) -> Self {
                Self(s)
            }
        }

        impl<S: ?Sized> CaseFold<S> {
            pub const fn borrow(s: &S) -> &Self {
                // SAFETY: #[repr(transparent)]
                unsafe { &*(s as *const S as *const Self) }
            }
        }

        impl<S: ?Sized + AsRef<str>> CaseFold<S> {
            pub fn as_str(&self) -> &str {
                self.0.as_ref()
            }
        }

        impl<'a, S: ?Sized> ToCaseFold<&'a CaseFold<S>> for &'a S {
            #[inline]
            fn to_case_fold(self) -> &'a CaseFold<S> {
                CaseFold::borrow(self)
            }
        }
        impl<'a, S: ?Sized> From<&'a S> for &'a CaseFold<S> {
            #[inline]
            fn from(value: &'a S) -> Self {
                value.to_case_fold()
            }
        }

        impl<S> ToCaseFold<CaseFold<S>> for S {
            #[inline]
            fn to_case_fold(self) -> CaseFold<Self> {
                CaseFold::new(self)
            }
        }

        impl<S> From<S> for CaseFold<S> {
            #[inline]
            fn from(value: S) -> Self {
                value.to_case_fold()
            }
        }

        impl<S: AsRef<$t>> std::borrow::Borrow<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn borrow(&self) -> &CaseFold<$t> {
                self.0.as_ref().to_case_fold()
            }
        }

        impl AsRef<CaseFold<$t>> for CaseFold<$t> {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self
            }
        }

        impl<S: AsRef<$t>> AsRef<CaseFold<$t>> for CaseFold<S> {
            #[inline]
            fn as_ref(&self) -> &CaseFold<$t> {
                self.0.as_ref().to_case_fold()
            }
        }

        impl<'a, S: ?Sized + AsRef<str>> Display for CaseFold<S> {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.as_str().fmt(f)
            }
        }
    };
}

pub mod ascii {
    use std::fmt::{self, Display, Formatter};
    use std::hash::{Hash, Hasher};
    use std::{iter, slice};

    use super::ToCaseFold;

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

    impl<S: ?Sized + AsRef<[u8]>> CaseFold<S> {
        #[inline]
        fn caseless_iter(&self) -> iter::Map<slice::Iter<'_, u8>, fn(&u8) -> u8> {
            self.0.as_ref().iter().map(u8::to_ascii_lowercase)
        }
    }

    impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
    where
        S: ?Sized + AsRef<[u8]>,
        Rhs: ?Sized + AsRef<[u8]>,
    {
        #[inline]
        fn eq(&self, other: &CaseFold<Rhs>) -> bool {
            self.0.as_ref().eq_ignore_ascii_case(other.0.as_ref())
        }
    }

    impl<S: ?Sized + AsRef<[u8]>> Hash for CaseFold<S> {
        #[inline]
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            for byte in self.caseless_iter() {
                hasher.write_u8(byte)
            }
        }
    }

    impl AsRef<CaseFold<[u8]>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().to_case_fold()
        }
    }

    impl AsRef<CaseFold<[u8]>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<[u8]> {
            self.as_bytes().to_case_fold()
        }
    }

    impl_ci!([u8]);
}

pub mod unicode {
    use std::char::ToLowercase;
    use std::fmt::{self, Display, Formatter};
    use std::hash::{Hash, Hasher};
    use std::iter;
    use std::str::Chars;

    use super::ToCaseFold;

    #[derive(Copy, Clone, Debug, Default)]
    #[repr(transparent)]
    pub struct CaseFold<S: ?Sized>(S);

    impl<S: ?Sized + AsRef<str>> CaseFold<S> {
        #[inline]
        fn caseless_iter(&self) -> iter::FlatMap<Chars, ToLowercase, fn(char) -> ToLowercase> {
            self.0.as_ref().chars().flat_map(char::to_lowercase)
        }
    }

    impl<S, Rhs> PartialEq<CaseFold<Rhs>> for CaseFold<S>
    where
        S: ?Sized + AsRef<str>,
        Rhs: ?Sized + AsRef<str>,
    {
        #[inline]
        fn eq(&self, other: &CaseFold<Rhs>) -> bool {
            self.caseless_iter().eq(other.caseless_iter())
        }
    }

    impl<S: ?Sized + AsRef<str>> Hash for CaseFold<S> {
        #[inline]
        fn hash<H: Hasher>(&self, hasher: &mut H) {
            for c in self.caseless_iter() {
                hasher.write_u32(c as u32)
            }
        }
    }

    impl AsRef<CaseFold<str>> for str {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.to_case_fold()
        }
    }

    impl AsRef<CaseFold<str>> for String {
        #[inline]
        fn as_ref(&self) -> &CaseFold<str> {
            self.as_str().to_case_fold()
        }
    }

    impl_ci!(str);
}
