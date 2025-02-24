//! Trait defining an unique symbolic identifier.

/// Trait defining an unique symbolic identifier.
pub trait Symbol: PartialEq + Eq + Clone + core::hash::Hash + core::fmt::Debug {}

impl<T> Symbol for T where T: PartialEq + Eq + Clone + core::hash::Hash + core::fmt::Debug {}
