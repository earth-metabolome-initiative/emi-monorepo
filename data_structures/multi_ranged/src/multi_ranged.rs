//! Submodule providing a `Range`-like struct with softer and stable
//! constraints.

use std::ops::{Mul, MulAssign};

use crate::{Step, errors::Error};

/// A trait for types that represent a range.
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

    /// Inserts a new element to the range.
    ///
    /// # Arguments
    ///
    /// * `element`: The element to add to the range.
    ///
    /// # Returns
    ///
    /// The range with the new element.
    ///
    /// # Errors
    ///
    /// * If the element cannot be added to the range.
    /// * If the element already exists in the range.
    fn insert(&mut self, element: Self::Step) -> Result<(), Error<Self::Step>>;

    /// Merges two ranges into one.
    ///
    /// # Arguments
    ///
    /// * `other`: The other range to merge with.
    ///
    /// # Returns
    ///
    /// The merged range.
    ///
    /// # Errors
    ///
    /// * If the ranges cannot be merged.
    fn merge<Rhs: MultiRanged<Step = Self::Step>>(
        &mut self,
        other: &Rhs,
    ) -> Result<(), Error<Self::Step>>;

    /// Returns whether the provided element is in the range.
    ///
    /// # Arguments
    ///
    /// * `element`: The element to check.
    ///
    /// # Returns
    ///
    /// `true` if the element is in the range, `false` otherwise.
    fn contains(&self, element: Self::Step) -> bool;

    /// Returns the start of the range.
    fn absolute_start(&self) -> Option<Self::Step>;

    /// Returns the end of the range.
    fn absolute_end(&self) -> Option<Self::Step>;

    /// Returns whether the range is dense.
    fn is_dense(&self) -> bool;
}
