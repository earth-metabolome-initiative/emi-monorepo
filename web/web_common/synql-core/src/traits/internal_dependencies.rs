//! Defines the `InternalDependencies` trait for retrieving internal crate
//! dependencies associated to an object.

use crate::structs::InternalCrate;

/// Returns the sorted unique internal crate dependencies associated to
/// the object.
pub trait InternalDependencies<'data> {
    /// Returns the sorted unique internal crate dependencies associated to
    /// the object.
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>>;
}
