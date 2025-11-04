//! Common trait for range types.

use std::ops::{Mul, MulAssign};

use crate::{Step, errors::Error};

/// Common interface for range data structures.
pub trait MultiRanged:
    core::fmt::Debug
    + Clone
    + Default
    + ExactSizeIterator<Item = <Self as MultiRanged>::Step>
    + DoubleEndedIterator<Item = <Self as MultiRanged>::Step>
    + Mul<Self::Step, Output = Self>
    + MulAssign<Self::Step>
    + TryFrom<(Self::Step, Self::Step), Error = Error<Self::Step>>
    + From<Self::Step>
{
    /// The type of the elements in the range.
    type Step: Step;

    /// Inserts an element into the range.
    ///
    /// # Errors
    ///
    /// Returns an error if the element cannot be added or already exists.
    fn insert(&mut self, element: Self::Step) -> Result<(), Error<Self::Step>>;

    /// Merges another range into this one.
    ///
    /// # Errors
    ///
    /// Returns an error if the ranges cannot be merged.
    fn merge<Rhs: MultiRanged<Step = Self::Step>>(
        &mut self,
        other: &Rhs,
    ) -> Result<(), Error<Self::Step>>;

    /// Returns whether the element is in the range.
    fn contains(&self, element: Self::Step) -> bool;

    /// Returns the start of the range, if it exists.
    fn absolute_start(&self) -> Option<Self::Step>;

    /// Returns the end of the range, if it exists.
    fn absolute_end(&self) -> Option<Self::Step>;

    /// Returns whether the range is contiguous (not split).
    fn is_dense(&self) -> bool;
}
