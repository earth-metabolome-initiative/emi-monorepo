//! Submodule defining the `Step` trait for number types.

use std::ops::{Mul, MulAssign};

use num_traits::{CheckedMul, ConstOne, ConstZero, SaturatingAdd, SaturatingSub, ToPrimitive};

/// A trait for types that can be used as steps in a range.
pub trait Step:
    ConstOne
    + ConstZero
    + Copy
    + PartialOrd
    + Ord
    + Mul<Output = Self>
    + MulAssign<Self>
    + CheckedMul
    + core::fmt::Debug
    + ToPrimitive
    + SaturatingAdd
    + SaturatingSub
{
    #[inline]
    #[must_use]
    /// Returns the next step after the current one.
    fn next(self) -> Self {
        self.saturating_add(&Self::ONE)
    }

    #[inline]
    #[must_use]
    /// Returns the previous step before the current one.
    fn prev(self) -> Self {
        self.saturating_sub(&Self::ONE)
    }
}

impl<T> Step for T where
    T: ConstOne
        + ConstZero
        + Copy
        + PartialOrd
        + Ord
        + Mul<Output = Self>
        + MulAssign<Self>
        + CheckedMul
        + core::fmt::Debug
        + ToPrimitive
        + SaturatingAdd
        + SaturatingSub
{
}
