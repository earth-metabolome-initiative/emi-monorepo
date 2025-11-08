//! Defines the `ExternalDependencies` trait for retrieving external crate
//! dependencies associated to an object.

use std::sync::Arc;

use crate::structs::ExternalCrate;

/// Returns the sorted unique external crate dependencies associated to
/// the object.
pub trait ExternalDependencies {
    /// Returns the sorted unique external crate dependencies associated to
    /// the object.
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>>;
}

impl<T: ExternalDependencies> ExternalDependencies for Option<T> {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        match self {
            Some(inner) => inner.external_dependencies(),
            None => vec![],
        }
    }
}

impl<T: ExternalDependencies> ExternalDependencies for Box<T> {
    fn external_dependencies(&self) -> Vec<Arc<ExternalCrate>> {
        self.as_ref().external_dependencies()
    }
}
