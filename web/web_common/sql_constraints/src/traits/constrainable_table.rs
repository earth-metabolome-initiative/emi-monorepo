//! Submodule providing the `ConstrainableTable` trait, which defines a
//! table-like object which can be constrained by SQL constraints.

use crate::traits::Constrainable;

/// Trait for types that define a table-like object which can be constrained by
/// SQL constraints.
pub trait ConstrainableTable: Constrainable {
    /// Get the name of the table.
    fn table_name(&self) -> &str;
}
