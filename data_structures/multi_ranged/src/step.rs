//! Trait for types that can be used as range boundaries.

use std::ops::{Mul, MulAssign};

use num_traits::{CheckedMul, ConstOne, ConstZero, SaturatingAdd, SaturatingSub, ToPrimitive};

/// Types that support stepping forward and backward with saturating arithmetic.
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
    /// Returns the next value.
    #[inline]
    #[must_use]
    fn next(self) -> Self {
        self.saturating_add(&Self::ONE)
    }

    /// Returns the previous value.
    #[inline]
    #[must_use]
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
