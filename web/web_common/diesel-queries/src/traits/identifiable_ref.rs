//! Submodule providing the `IdentifiableRef` trait for Diesel queries.

use std::borrow::Borrow;

use diesel::Identifiable;

/// Trait representing that an object has a reference which implements
/// `Identifiable`.
pub trait IdentifiableRef {
    /// The identifier type of the reference.
    type Id<'a>
    where
        Self: 'a;

    /// The type of the extended object.
    type RefType<'data>: Borrow<Self> + Identifiable<Id = Self::Id<'data>>
    where
        Self: 'data;
}

impl<T> IdentifiableRef for T
where
    for<'a> &'a T: Identifiable,
{
    type Id<'a>
        = <&'a T as Identifiable>::Id
    where
        Self: 'a;

    type RefType<'data>
        = &'data Self
    where
        Self: 'data;
}
