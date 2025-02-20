//! Defines the properties of a numeric identifier for an edge or node in a graph.


/// A trait defining the properties of a numeric identifier for an edge or node in a graph.
pub trait NumericIdentifier:
    core::fmt::Debug + core::fmt::Display + Copy + PartialEq + Eq + PartialOrd + Ord
{
}

impl NumericIdentifier for u8 {}
impl NumericIdentifier for u16 {}
impl NumericIdentifier for u32 {}
impl NumericIdentifier for u64 {}
impl NumericIdentifier for u128 {}
