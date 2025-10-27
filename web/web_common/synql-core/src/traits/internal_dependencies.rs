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

impl<'data, T: InternalDependencies<'data>> InternalDependencies<'data> for Option<T> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        match self {
            Some(inner) => inner.internal_dependencies(),
            None => vec![],
        }
    }
}

impl<'data, T: InternalDependencies<'data>> InternalDependencies<'data> for Box<T> {
    fn internal_dependencies(&self) -> Vec<&InternalCrate<'data>> {
        self.as_ref().internal_dependencies()
    }
}
