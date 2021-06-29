use std::cmp::{min, Ordering};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::{ExactSizeIterator, FilterMap, FromIterator, FusedIterator, Iterator, Zip};
use std::marker::PhantomData;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Not};
use std::{slice, vec};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub trait Enum: Copy + Ord {
    /// The bitwise representation
    type Rep;
    const SIZE: usize;
    /// The smallest value in the type.
    /// Rule: there should be no value `x` such that `x.succ() == Some(Self::MIN).
    const MIN: Self;
    /// The largest value in the type.
    /// Rule: `Self::max_value().succ() == None`.
    const MAX: Self;

    /// Returns the next value after `self`, or `None` if `self == Self::MAX`.
    fn succ(self) -> Option<Self>;
    fn bit(self) -> Self::Rep;
    fn index(self) -> usize;
    fn enumerate() -> Enumeration<Self> {
        Enumeration(Some(Self::MIN))
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
    fn bit(self) -> Self::Rep {
        1 << (self as u8)
    }
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enumeration<T>(Option<T>);

impl<T: Enum> Iterator for Enumeration<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let inner = self.0?;
        self.0 = inner.succ();
        Some(inner)
    }

    fn count(self) -> usize {
        match self.0 {
            None => 0,
            Some(x) => T::SIZE - x.index() - 1,
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let exact = self.count();
        (exact, Some(exact))
    }
}
impl<T: Enum> FusedIterator for Enumeration<T> {}
impl<T: Enum> ExactSizeIterator for Enumeration<T> {
    fn len(&self) -> usize {
        self.count()
    }
}

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
        impl<T: Enum> std::ops::$t for EnumSet<T>
        where
            T::Rep: std::ops::$t<Output = T::Rep>,
        {
            type Output = Self;
            #[inline]
            fn $f(self, other: Self) -> Self::Output {
                Self {
                    raw: self.raw.$f(other.raw),
                }
            }
        }
        impl<T: Enum> std::ops::$t<T> for EnumSet<T>
        where
            T::Rep: std::ops::$t<Output = T::Rep>,
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
        impl<T: Enum> std::ops::$t for EnumSet<T>
        where
            T::Rep: std::ops::$t,
        {
            #[inline]
            fn $f(&mut self, other: Self) {
                self.raw.$f(other.raw)
            }
        }
        impl<T: Enum> std::ops::$t<T> for EnumSet<T>
        where
            T::Rep: std::ops::$t,
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
            iter: T::enumerate(),
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnumMap<K, V> {
    inner: Vec<Option<V>>,
    marker: PhantomData<K>,
}

impl<K: Enum, V> Default for EnumMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Enum, V> EnumMap<K, V> {
    pub fn new() -> Self {
        let mut inner = Vec::with_capacity(K::SIZE);
        inner.resize_with(K::SIZE, Default::default);
        Self {
            inner,
            marker: PhantomData,
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner[k.index()].replace(v)
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        self.inner[k.index()].take()
    }
    pub fn get(&self, k: K) -> Option<&V> {
        self.inner[k.index()].as_ref()
    }
    pub fn get_mut(&mut self, k: K) -> Option<&mut V> {
        self.inner[k.index()].as_mut()
    }
}

impl<K: Enum + Copy + Ord, V> EnumMap<K, V> {
    pub fn keys(&self) -> impl '_ + Iterator<Item = K> {
        K::enumerate().filter(move |x| self.inner[x.index()].is_some())
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.inner.iter().filter_map(Option::as_ref)
    }

    pub fn iter(&self) -> impl Iterator<Item = (K, &V)> {
        (&self).into_iter()
    }

    pub fn iter_copied(&self) -> impl '_ + Iterator<Item = (K, V)>
    where
        V: Copy,
    {
        K::enumerate()
            .zip(&self.inner)
            .filter_map(|(x, m_y)| m_y.map(|y| (x, y)))
    }
}

impl<'de, K, V> Deserialize<'de> for EnumMap<K, V>
where
    K: Enum + Deserialize<'de> + Copy + Ord,
    V: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct MapVisitor<K, V> {
            marker: PhantomData<EnumMap<K, V>>,
        }

        impl<'de, K, V> Visitor<'de> for MapVisitor<K, V>
        where
            K: Enum + Deserialize<'de>,
            V: Deserialize<'de>,
        {
            type Value = EnumMap<K, V>;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("a map")
            }

            #[inline]
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut values = EnumMap::new();
                while let Some((k, v)) = map.next_entry()? {
                    values.insert(k, v);
                }
                Ok(values)
            }
        }

        let visitor = MapVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_map(visitor)
    }
}

impl<K, V> Serialize for EnumMap<K, V>
where
    K: Enum + Serialize + Copy + Ord,
    V: Serialize,
{
    #[inline]
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_map(self)
    }
}

impl<K: Enum + Copy + Ord, V> FromIterator<(K, V)> for EnumMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut this = EnumMap::new();
        for (k, v) in iter {
            this.insert(k, v);
        }
        this
    }
}

impl<K: Enum + Copy + Ord, V> IntoIterator for EnumMap<K, V> {
    type Item = (K, V);
    #[allow(clippy::type_complexity)]
    type IntoIter = FilterMap<
        Zip<Enumeration<K>, vec::IntoIter<Option<V>>>,
        fn((K, Option<V>)) -> Option<(K, V)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        K::enumerate()
            .zip(self.inner)
            .filter_map(|(x, m_y)| m_y.map(|y| (x, y)))
    }
}

impl<'a, K: Enum + Copy + Ord, V> IntoIterator for &'a EnumMap<K, V> {
    type Item = (K, &'a V);
    #[allow(clippy::type_complexity)]
    type IntoIter = FilterMap<
        Zip<Enumeration<K>, slice::Iter<'a, Option<V>>>,
        fn((K, &Option<V>)) -> Option<(K, &V)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        K::enumerate()
            .zip(&self.inner)
            .filter_map(|(x, m_y)| m_y.as_ref().map(|y| (x, y)))
    }
}

impl<'a, K: Enum + Copy + Ord, V> IntoIterator for &'a mut EnumMap<K, V> {
    type Item = (K, &'a mut V);
    #[allow(clippy::type_complexity)]
    type IntoIter = FilterMap<
        Zip<Enumeration<K>, slice::IterMut<'a, Option<V>>>,
        fn((K, &mut Option<V>)) -> Option<(K, &mut V)>,
    >;

    fn into_iter(self) -> Self::IntoIter {
        K::enumerate()
            .zip(&mut self.inner)
            .filter_map(|(x, m_y)| m_y.as_mut().map(|y| (x, y)))
    }
}
