//! Submodule defining the Integer number trait.

use num_traits::{CheckedMul, SaturatingAdd, SaturatingSub, ToPrimitive};

/// Trait defining an integer number.
pub trait Integer:
    super::number::Number
    + Ord
    + Eq
    + core::hash::Hash
    + From<bool>
    + SaturatingAdd
    + SaturatingSub
    + CheckedMul
    + ToPrimitive
{
}

impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for u128 {}
impl Integer for usize {}
