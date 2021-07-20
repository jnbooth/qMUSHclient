use std::cmp::{min, Ordering};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{ExactSizeIterator, FromIterator, FusedIterator, Iterator};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

use super::{Enum, Enumeration};

pub trait Wordlike {
    const ZERO: Self;
    // This is an associated function in order to avoid conflict with a method on the inner type.
    fn count_ones(this: Self) -> u32;
}

macro_rules! impl_word {
    ($n: ty) => {
        impl Wordlike for $n {
            const ZERO: Self = 0;
            #[inline]
            fn count_ones(this: Self) -> u32 {
                this.count_ones()
            }
        }
    };
}

impl_word!(isize);
impl_word!(u128);
impl_word!(u16);
impl_word!(i128);
impl_word!(i16);
impl_word!(u64);
impl_word!(u8);
impl_word!(i64);
impl_word!(i8);
impl_word!(u32);
impl_word!(usize);
impl_word!(i32);

#[repr(transparent)]
pub struct EnumSet<T: Enum> {
    // FIXME(#57563) replace with `const fn from_raw(T::Rep)` if const_fn_trait_bound is stabilized
    pub raw: T::Rep,
}

impl<T: Enum> Copy for EnumSet<T> where T::Rep: Copy {}

impl<T: Enum> Clone for EnumSet<T>
where
    T::Rep: Copy,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Enum> PartialEq for EnumSet<T>
where
    T::Rep: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.raw == other.raw
    }
}
impl<T: Enum> Eq for EnumSet<T> where T::Rep: Eq {}

impl<T: Enum> PartialOrd for EnumSet<T>
where
    T::Rep: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.raw.partial_cmp(&other.raw)
    }
}

impl<T: Enum> Ord for EnumSet<T>
where
    T::Rep: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T: Enum> Hash for EnumSet<T>
where
    T::Rep: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state);
    }
}

impl<T: Enum> Not for EnumSet<T>
where
    T::Rep: Not<Output = T::Rep>,
{
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self { raw: !self.raw }
    }
}

macro_rules! bitop {
    ($t:tt, $f:ident) => {
        impl<T: Enum> $t for EnumSet<T>
        where
            T::Rep: $t<Output = T::Rep>,
        {
            type Output = Self;
            #[inline]
            fn $f(self, other: Self) -> Self::Output {
                Self {
                    raw: self.raw.$f(other.raw),
                }
            }
        }
        impl<T: Enum> $t<T> for EnumSet<T>
        where
            T::Rep: $t<Output = T::Rep>,
        {
            type Output = Self;
            #[inline]
            fn $f(self, other: T) -> Self::Output {
                Self {
                    raw: self.raw.$f(other.bit()),
                }
            }
        }
    };
}
macro_rules! bitassign {
    ($t:tt, $f:ident) => {
        impl<T: Enum> $t for EnumSet<T>
        where
            T::Rep: $t,
        {
            #[inline]
            fn $f(&mut self, other: Self) {
                self.raw.$f(other.raw)
            }
        }
        impl<T: Enum> $t<T> for EnumSet<T>
        where
            T::Rep: $t,
        {
            #[inline]
            fn $f(&mut self, other: T) {
                self.raw.$f(other.bit())
            }
        }
    };
}
bitop!(BitAnd, bitand);
bitassign!(BitAndAssign, bitand_assign);
bitop!(BitOr, bitor);
bitassign!(BitOrAssign, bitor_assign);
bitop!(BitXor, bitxor);
bitassign!(BitXorAssign, bitxor_assign);

impl<T: Enum> FromIterator<T> for EnumSet<T>
where
    T::Rep: BitOr<Output = T::Rep> + Wordlike,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            raw: iter
                .into_iter()
                .map(T::bit)
                .fold(Wordlike::ZERO, BitOr::bitor),
        }
    }
}

impl<'a, T: Enum + Copy> FromIterator<&'a T> for EnumSet<T>
where
    T::Rep: BitOr<Output = T::Rep> + Wordlike,
{
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        Self {
            raw: iter
                .into_iter()
                .map(|&x| T::bit(x))
                .fold(Wordlike::ZERO, BitOr::bitor),
        }
    }
}

impl<T: Enum> Default for EnumSet<T>
where
    T::Rep: Wordlike,
{
    fn default() -> Self {
        Self::new()
    }
}

#[macro_use]
macro_rules! enums {
    () => ($crate::enums::EnumSet{raw: $crate::enums::Wordlike::ZERO});
    ($($i:expr),+ $(,)?) => ({
        #[cfg(debug_assertions)]
        let _ = [$($i),+]; // all items are same type
        #[allow(unused_imports)]
        use $crate::enums::{Enum, EnumSet};
        EnumSet{raw: 0$(|$i.bit())*}
    });
}

impl<T: Enum> EnumSet<T>
where
    T::Rep: Wordlike,
{
    #[inline]
    pub fn new() -> Self {
        enums![]
    }

    pub fn clear(&mut self) {
        self.raw = Wordlike::ZERO;
    }

    pub fn insert(&mut self, x: T)
    where
        T::Rep: BitOrAssign,
    {
        self.raw |= x.bit()
    }

    pub fn remove(&mut self, x: T)
    where
        T::Rep: BitAndAssign + Not<Output = T::Rep>,
    {
        self.raw &= !x.bit()
    }
    pub fn contains(&self, x: T) -> bool
    where
        T::Rep: BitAnd<Output = T::Rep> + Eq + Copy,
    {
        self.raw & x.bit() != Wordlike::ZERO
    }
    pub fn into_raw(self) -> T::Rep {
        self.raw
    }
    pub fn from_raw(raw: T::Rep) -> Self {
        Self { raw }
    }
}

impl<T: Enum + Copy + Ord> IntoIterator for EnumSet<T>
where
    T::Rep: BitAnd<Output = T::Rep> + Wordlike + Eq + Copy,
{
    type Item = T;
    type IntoIter = EnumIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        EnumIter {
            set: self,
            iter: T::enumerate(..),
        }
    }
}

impl<T: Debug + Enum> Debug for EnumSet<T>
where
    EnumSet<T>: IntoIterator<Item = T> + Copy,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.into_iter()).finish()
    }
}

pub struct EnumIter<T: Enum> {
    set: EnumSet<T>,
    iter: Enumeration<T>,
}

impl<T: Enum + Copy + Ord> Iterator for EnumIter<T>
where
    T::Rep: BitAnd<Output = T::Rep> + Wordlike + Eq + Copy,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let set = self.set;
        self.iter.find(move |&x| set.contains(x))
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = Wordlike::count_ones(self.set.raw) as usize;
        (0, Some(min(self.iter.len(), count)))
    }

    #[inline]
    fn count(self) -> usize {
        let set = self.set;
        self.iter.map(move |x| set.contains(x) as usize).sum()
    }

    #[inline]
    fn fold<Acc, Fold>(self, init: Acc, mut fold: Fold) -> Acc
    where
        Fold: FnMut(Acc, Self::Item) -> Acc,
    {
        let set = self.set;
        self.iter.fold(init, move |acc, item| {
            if set.contains(item) {
                fold(acc, item)
            } else {
                acc
            }
        })
    }
}

impl<T: Enum> FusedIterator for EnumIter<T> where EnumIter<T>: Iterator<Item = T> {}

#[cfg(test)]
mod tests {
    use super::super::tests::*;
    use super::*;

    // EnumSet tests

    #[test]
    fn test_enumerate() {
        assert_eq!(
            EnumSet { raw: !0 }.into_iter().collect::<Vec<DemoEnum>>(),
            Enum::enumerate(..).collect::<Vec<DemoEnum>>()
        );
    }
}
