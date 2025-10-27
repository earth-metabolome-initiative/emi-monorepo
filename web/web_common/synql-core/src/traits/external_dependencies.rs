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

impl<'data, T: ExternalDependencies<'data>> ExternalDependencies<'data> for Option<T> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        match self {
            Some(inner) => inner.external_dependencies(),
            None => vec![],
        }
    }
}

impl<'data, T: ExternalDependencies<'data>> ExternalDependencies<'data> for Box<T> {
    fn external_dependencies(&self) -> Vec<&ExternalCrate<'data>> {
        self.as_ref().external_dependencies()
    }
}