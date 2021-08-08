use std::fmt::{self, Debug, Formatter};
use std::hash::Hash;
use std::iter::{FilterMap, FromIterator, Iterator, Zip};
use std::marker::PhantomData;
use std::{slice, vec};

use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Enum, Enumeration};

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
        K::enumerate(..).filter(move |x| self.inner[x.index()].is_some())
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
        K::enumerate(..)
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
        K::enumerate(..)
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
        K::enumerate(..)
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
        K::enumerate(..)
            .zip(&mut self.inner)
            .filter_map(|(x, m_y)| m_y.as_mut().map(|y| (x, y)))
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use super::super::tests::*;
}
