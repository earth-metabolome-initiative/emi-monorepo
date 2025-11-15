//! Defines the `ExternalDependencies` trait for retrieving external crate
//! dependencies associated to an object.

use crate::structs::ExternalCrate;

/// Returns the external crate dependencies associated to
/// the object.
pub trait ExternalDependencies {
    /// Returns the external crate dependencies associated to
    /// the object.
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate>;
}

impl<T: ExternalDependencies> ExternalDependencies for Option<T> {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.as_ref().into_iter().flat_map(ExternalDependencies::external_dependencies)
    }
}

impl<T: ExternalDependencies> ExternalDependencies for Box<T> {
    #[inline]
    fn external_dependencies(&self) -> impl Iterator<Item = &ExternalCrate> {
        self.as_ref().external_dependencies()
    }
}
