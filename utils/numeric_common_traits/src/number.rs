//! Trait defining a number.

use common_traits::prelude::{Serde, TotalOrd};
use num_traits::{ConstOne, ConstZero};

use super::prelude::{Bounded, Finite, Ten, Two};

/// Trait defining a number.
pub trait Number:
    Copy
    + Clone
    + core::fmt::Display
    + core::fmt::Debug
    + PartialEq
    + PartialOrd
    + TotalOrd
    + Ten
    + Two
    + ConstOne
    + ConstZero
    + Finite
    + Bounded
    + Serde
    + core::iter::Sum
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Div<Output = Self>
    + core::ops::Rem<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
    + core::ops::DivAssign
    + core::ops::RemAssign
{
}

/// Trait defining a positive number.
pub trait PositiveNumber: Number {}

impl<T> Number for T where
    T: Copy
        + Clone
        + core::fmt::Display
        + core::fmt::Debug
        + PartialEq
        + PartialOrd
        + TotalOrd
        + Ten
        + Two
        + ConstOne
        + ConstZero
        + Finite
        + Bounded
        + Serde
        + core::iter::Sum
        + core::ops::Add<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Div<Output = T>
        + core::ops::Rem<Output = T>
        + core::ops::AddAssign
        + core::ops::SubAssign
        + core::ops::MulAssign
        + core::ops::DivAssign
        + core::ops::RemAssign
{
}
