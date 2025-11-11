//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::Identifiable;

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended>
where
    for<'ext> &'ext Self: Identifiable,
{
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
        + Identifiable<Id = <&'data Self as Identifiable>::Id>
    where
        Self: 'data;
}

/// Trait representing an object that can retrieve its ancestor
/// (the extended object) from a database connection.
pub trait Ancestor<Extended, C>: ExtensionOf<Extended>
where
    for<'ext> &'ext Self: Identifiable,
{
    /// Returns the extended object.
    fn ancestor(&self, connection: &mut C)
    -> Result<Self::ExtendedType<'_>, diesel::result::Error>;
}
