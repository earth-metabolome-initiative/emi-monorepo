//! Defines the `InternalDependencies` trait for retrieving internal crate
//! dependencies associated to an object.

use crate::structs::InternalCrate;

/// Returns the sorted unique internal crate dependencies associated to
/// the object.
pub trait InternalDependencies {
    /// Returns the sorted unique internal crate dependencies associated to
    /// the object.
    fn internal_dependencies(&self) -> Vec<&InternalCrate>;
}

impl<T: InternalDependencies> InternalDependencies for Option<T> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate> {
        match self {
            Some(inner) => inner.internal_dependencies(),
            None => vec![],
        }
    }
}

impl<T: InternalDependencies> InternalDependencies for Box<T> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate> {
        self.as_ref().internal_dependencies()
    }
}
