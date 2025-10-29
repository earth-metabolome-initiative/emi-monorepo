//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended, C> {
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
    where
        Self: 'data;

    /// Returns the extended object.
    fn ancestor(&self, connection: &mut C)
    -> Result<Self::ExtendedType<'_>, diesel::result::Error>;
}
