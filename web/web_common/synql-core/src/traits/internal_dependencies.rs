//! Defines the `InternalDependencies` trait for retrieving internal crate
//! dependencies associated to an object.

use crate::structs::InternalCrate;

/// Returns the internal crate dependencies associated to
/// the object.
pub trait InternalDependencies {
    /// Returns the internal crate dependencies associated to
    /// the object.
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate>;
}

impl<T: InternalDependencies> InternalDependencies for Option<T> {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.into_iter().flat_map(|item| item.internal_dependencies())
    }
}

impl<T: InternalDependencies> InternalDependencies for Box<T> {
    #[inline]
    fn internal_dependencies(&self) -> impl Iterator<Item = &InternalCrate> {
        self.as_ref().internal_dependencies()
    }
}
