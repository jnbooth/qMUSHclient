mod array;
mod maybe_uninit;
use array::collect_into_array;

use std::iter::{ExactSizeIterator, FusedIterator};

pub struct ChunkIter<I: Iterator, const N: usize>(I);

impl<I: Iterator, const N: usize> Iterator for ChunkIter<I, N> {
    type Item = [I::Item; N];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        collect_into_array(&mut self.0)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lower, upper) = self.0.size_hint();
        (lower / N, upper.map(|x| x / N))
    }

    #[inline]
    fn count(self) -> usize {
        self.0.count() / N
    }
}

impl<I: Iterator + FusedIterator, const N: usize> FusedIterator for ChunkIter<I, N> {}
impl<I: Iterator + ExactSizeIterator, const N: usize> ExactSizeIterator for ChunkIter<I, N> {
    #[inline]
    fn len(&self) -> usize {
        self.0.len() / N
    }
}

pub trait IterChunks: Iterator + Sized {
    #[inline]
    fn chunks<const N: usize>(self) -> ChunkIter<Self, N> {
        ChunkIter(self)
    }
}
impl<T: Iterator + Sized> IterChunks for T {}
