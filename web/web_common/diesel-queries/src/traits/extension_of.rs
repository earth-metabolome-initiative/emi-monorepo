//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::Identifiable;

use crate::traits::IdentifiableRef;

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended>: IdentifiableRef {
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
        + Identifiable<Id = <Self as IdentifiableRef>::Id<'data>>
    where
        Self: 'data;
}

/// Trait representing an object that can retrieve its ancestor
/// (the extended object) from a database connection.
pub trait Ancestor<Extended, C>: ExtensionOf<Extended> {
    /// Returns the extended object.
    fn ancestor(&self, connection: &mut C)
    -> Result<Self::ExtendedType<'_>, diesel::result::Error>;
}
