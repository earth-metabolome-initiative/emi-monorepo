//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::Identifiable;

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOfHelper<Extended> {}

impl<Extended, T> ExtensionOfHelper<Extended> for T
where
    for<'a> &'a Extended: Identifiable,
    for<'a> &'a T: Identifiable<Id = <&'a Extended as Identifiable>::Id>,
{
}

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended>: ExtensionOfHelper<Extended> {
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
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
