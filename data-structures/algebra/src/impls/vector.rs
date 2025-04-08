//! Implementations of the [`Vector`] trait for various types.

use crate::traits::Vector;

impl<V> Vector for [V] {
    type Value = V;
    type Iter<'a>
        = core::slice::Iter<'a, V>
    where
        Self: 'a;
    type Index = usize;
    type Indices<'a>
        = core::ops::Range<usize>
    where
        Self: 'a;

    fn indices(&self) -> Self::Indices<'_> {
        0..self.len()
    }

    fn len(&self) -> usize {
        <[V]>::len(self)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.iter()
    }
}

impl<V> Vector for Vec<V> {
    type Value = V;
    type Iter<'a>
        = core::slice::Iter<'a, V>
    where
        Self: 'a;
    type Index = usize;
    type Indices<'a>
        = core::ops::Range<usize>
    where
        Self: 'a;

    fn indices(&self) -> Self::Indices<'_> {
        0..self.len()
    }

    fn len(&self) -> usize {
        Vec::len(self)
    }

    fn iter(&self) -> Self::Iter<'_> {
        self.as_slice().iter()
    }
}

impl<V, const N: usize> Vector for [V; N] {
    type Value = V;
    type Iter<'a>
        = core::slice::Iter<'a, V>
    where
        Self: 'a;
    type Index = usize;
    type Indices<'a>
        = core::ops::Range<usize>
    where
        Self: 'a;

    fn indices(&self) -> Self::Indices<'_> {
        0..N
    }

    fn len(&self) -> usize {
        N
    }

    fn iter(&self) -> Self::Iter<'_> {
        <[V]>::iter(self)
    }
}
