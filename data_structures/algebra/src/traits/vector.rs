//! Submodule providing the [`Vector`] trait.

use core::ops::Index;

use num_traits::ConstZero;
use numeric_common_traits::prelude::{IntoUsize, PositiveInteger};

/// Trait defining a vector.
pub trait Vector: Index<<Self as Vector>::Index, Output = <Self as Vector>::Value> {
    /// The value of the vector.
    type Value;
    /// Iterator over the values in the vector.
    type Iter<'a>: Iterator<Item = &'a Self::Value> + Clone + DoubleEndedIterator
    where
        Self: 'a;
    /// The index of the vector.
    type Index: PositiveInteger + IntoUsize;
    /// Iterator over the indices of the vector.
    type Indices<'a>: Iterator<Item = Self::Index>
    where
        Self: 'a;

    /// Returns an iterator over the values in the vector.
    fn iter(&self) -> Self::Iter<'_>;

    /// Returns an iterator over the indices of the vector.
    fn indices(&self) -> Self::Indices<'_>;

    /// Returns the number of elements in the vector.
    fn len(&self) -> Self::Index;

    /// Returns whether the vector is empty.
    fn is_empty(&self) -> bool {
        self.len() == <Self::Index as ConstZero>::ZERO
    }
}
