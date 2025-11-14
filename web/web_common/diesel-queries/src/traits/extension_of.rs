//! Submodule providing the `ExtensionOf` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::{Identifiable, associations::HasTable};

/// Trait representing an object that is an extension of another object.
pub trait TableIsExtensionOf<Extended: diesel::Table>: diesel::Table {}
impl<T: diesel::Table> TableIsExtensionOf<T> for T {}

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOfHelper<Extended: HasTable>: HasTable {}

impl<Extended: HasTable, T: HasTable> ExtensionOfHelper<Extended> for T
where
    T::Table: TableIsExtensionOf<<Extended as HasTable>::Table>,
    for<'a> &'a Extended: Identifiable,
    for<'a> &'a T: Identifiable<Id = <&'a Extended as Identifiable>::Id>,
{
}

/// Trait representing an object that is an extension of another object.
pub trait ExtensionOf<Extended: HasTable>: ExtensionOfHelper<Extended> {
    /// The type of the extended object.
    type ExtendedType<'data>: Borrow<Extended>
    where
        Self: 'data;
}

impl<T: ExtensionOfHelper<T>> ExtensionOf<T> for T {
    type ExtendedType<'data>
        = &'data T
    where
        Self: 'data;
}

/// Trait representing an object that can retrieve its ancestor
/// (the extended object) from a database connection.
pub trait Ancestor<Extended: HasTable, C>: ExtensionOf<Extended> {
    /// Returns the extended object.
    fn ancestor(&self, connection: &mut C)
    -> Result<Self::ExtendedType<'_>, diesel::result::Error>;
}

impl<T, C> Ancestor<T, C> for T
where
    T: ExtensionOfHelper<T>,
{
    fn ancestor(
        &self,
        _connection: &mut C,
    ) -> Result<Self::ExtendedType<'_>, diesel::result::Error> {
        Ok(self)
    }
}
