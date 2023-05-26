use std::borrow::Borrow;
use std::collections::hash_map::{Entry, HashMap, RandomState};
use std::fmt::{self, Debug, Formatter};
use std::hash::{BuildHasher, Hash};
use std::iter::FromIterator;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

pub struct AsRefHashMap<R: ?Sized, K, V, S = RandomState>(HashMap<K, V, S>, PhantomData<R>);

impl<R: ?Sized, K: Eq + Hash, V: PartialEq, S: BuildHasher> PartialEq for AsRefHashMap<R, K, V, S> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<R: ?Sized, K: Clone, V: Clone, S: Clone> Clone for AsRefHashMap<R, K, V, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), PhantomData)
    }
}

impl<R: ?Sized, K: Debug, V: Debug, S> Debug for AsRefHashMap<R, K, V, S> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<R: ?Sized, K: Eq + Hash, V: Eq, S: BuildHasher> Eq for AsRefHashMap<R, K, V, S> {}

impl<R: ?Sized, K, V, S: Default> Default for AsRefHashMap<R, K, V, S> {
    fn default() -> Self {
        Self(HashMap::with_hasher(S::default()), PhantomData)
    }
}

impl<R: ?Sized, K, V, S: Default + BuildHasher> AsRefHashMap<R, K, V, S> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(
            HashMap::with_capacity_and_hasher(capacity, S::default()),
            PhantomData,
        )
    }
}

impl<R: ?Sized, K, V, S> AsRefHashMap<R, K, V, S> {
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        Self(
            HashMap::with_capacity_and_hasher(capacity, hash_builder),
            PhantomData,
        )
    }
}

impl<R: ?Sized + Eq + Hash, K: Eq + Hash + Borrow<R>, V, S: BuildHasher> AsRefHashMap<R, K, V, S> {
    #[inline]
    pub fn entry<Q>(&mut self, k: Q) -> Entry<K, V>
    where
        Q: Into<K>,
    {
        self.0.entry(k.into())
    }

    #[inline]
    pub fn insert<Q>(&mut self, k: Q, v: V) -> Option<V>
    where
        Q: Into<K>,
    {
        self.0.insert(k.into(), v)
    }

    #[inline]
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get(k.as_ref())
    }

    #[inline]
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get_mut(k.as_ref())
    }

    #[inline]
    pub fn get_key_value<Q>(&self, k: &Q) -> Option<(&K, &V)>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.get_key_value(k.as_ref())
    }

    #[inline]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.contains_key(k.as_ref())
    }

    #[inline]
    pub fn remove<Q>(&mut self, k: &Q) -> Option<V>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.remove(k.as_ref())
    }

    #[inline]
    pub fn remove_entry<Q>(&mut self, k: &Q) -> Option<(K, V)>
    where
        Q: ?Sized + AsRef<R>,
    {
        self.0.remove_entry(k.as_ref())
    }
}

impl<R: ?Sized, K, V, S> Deref for AsRefHashMap<R, K, V, S> {
    type Target = HashMap<K, V, S>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: ?Sized, K, V, S> DerefMut for AsRefHashMap<R, K, V, S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R, K, V, S> FromIterator<(K, V)> for AsRefHashMap<R, K, V, S>
where
    R: ?Sized,
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self(HashMap::from_iter(iter), PhantomData)
    }
}
