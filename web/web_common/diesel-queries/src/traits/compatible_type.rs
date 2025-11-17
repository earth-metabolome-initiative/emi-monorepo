//! Defines a trait for determing whether two types are compatible
//! for a foreign key relationship in Diesel schemas.

use diesel::dsl::Nullable;

/// Defines a trait for determining whether two types are compatible
/// for a foreign key relationship in Diesel.
pub trait ForeignKeyCompatibleType<Other> {}

impl<T> ForeignKeyCompatibleType<T> for T {}
impl<T> ForeignKeyCompatibleType<Nullable<T>> for T {}
impl<T> ForeignKeyCompatibleType<T> for Nullable<T> {}

/// Helper trait for a single compatible type.
pub trait ForeignKeyCompatibleColumn<Other> {}

impl<C1, C2> ForeignKeyCompatibleColumn<C2> for C1
where
    C1: diesel::Column,
    C2: diesel::Column,
    C1::SqlType: ForeignKeyCompatibleType<C2::SqlType>,
{
}

/// Helper trait for tuples of compatible types.
pub trait ForeignKeyCompatibleColumns<Other> {}

impl<C1, C2> ForeignKeyCompatibleColumns<(C1,)> for (C2,) where C2: ForeignKeyCompatibleColumn<C1> {}

impl<C1, C2, C3, C4> ForeignKeyCompatibleColumns<(C1, C2)> for (C3, C4)
where
    C3: ForeignKeyCompatibleColumn<C1>,
    C4: ForeignKeyCompatibleColumn<C2>,
{
}

impl<C1, C2, C3, C4, C5, C6> ForeignKeyCompatibleColumns<(C1, C2, C3)> for (C4, C5, C6)
where
    C4: ForeignKeyCompatibleColumn<C1>,
    C5: ForeignKeyCompatibleColumn<C2>,
    C6: ForeignKeyCompatibleColumn<C3>,
{
}
