//! Submodule defining the properties of a positive integer.

/// Trait defining a positive integer.
pub trait PositiveInteger: super::Integer {}

impl PositiveInteger for u8 {}
impl PositiveInteger for u16 {}
impl PositiveInteger for u32 {}
impl PositiveInteger for u64 {}
impl PositiveInteger for u128 {}
impl PositiveInteger for usize {}
