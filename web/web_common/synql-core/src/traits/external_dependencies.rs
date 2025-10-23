//! Defines the `ExternalDependencies` trait for retrieving external crate
//! dependencies associated to an object.

use crate::structs::ExternalCrate;

/// Returns the sorted unique external crate dependencies associated to
/// the object.
pub trait ExternalDependencies<'data> {
    /// Returns the sorted unique external crate dependencies associated to
    /// the object.
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>>;
}
