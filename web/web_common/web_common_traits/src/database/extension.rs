//! Submodule defining the `ExtensionTable` trait for struct tables.

use diesel::Identifiable;

/// Trait marker defining that a table is an extension of another table.
pub trait ExtensionTable<T>
where
    for<'a> &'a T: Identifiable,
    for<'a> &'a Self: Identifiable<Id = <&'a T as Identifiable>::Id>,
{
}

impl<T, C> ExtensionTable<T> for &C
where
    Self: ExtensionTable<T>,
    for<'a> &'a T: Identifiable,
    for<'a> &'a Self: Identifiable<Id = <&'a T as Identifiable>::Id>,
{
}
