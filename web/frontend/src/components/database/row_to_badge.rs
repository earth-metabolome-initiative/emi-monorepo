//! Submodule defining a trait to be implemented by types that can be converted to a badge.

/// Trait for types that can be converted to a badge.
pub trait RowToBadge {
    /// The badge type that the implementing type can be converted to.
    type Badge;

    /// Convert the implementing type to a badge.
    fn to_badge(&self) -> Self::Badge;
}