//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::Identifiable;

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended: Identifiable>: Identifiable<Id = Extended::Id> {
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
    where
        Self: 'data;
}

/// Trait representing an object that can retrieve its ancestor
/// (the extended object) from a database connection.
pub trait Ancestor<Extended: Identifiable, C>: ExtensionOf<Extended> {
    /// Returns the extended object.
    fn ancestor(&self, connection: &mut C)
    -> Result<Self::ExtendedType<'_>, diesel::result::Error>;
}
