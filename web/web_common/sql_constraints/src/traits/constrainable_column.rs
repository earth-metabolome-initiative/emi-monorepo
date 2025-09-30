//! Submodule providing the `ConstrainableColumn` trait, which defines a
//! column-like object which can be constrained by SQL constraints.

use crate::traits::Constrainable;

/// Trait for types that define a column-like object which can be constrained by
/// SQL constraints.
pub trait ConstrainableColumn: Constrainable {
    /// Get the name of the column.
    fn column_name(&self) -> &str;
}
