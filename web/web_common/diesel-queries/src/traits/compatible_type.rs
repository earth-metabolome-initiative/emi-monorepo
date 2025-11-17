//! Defines a trait for determing whether two types are compatible
//! for a foreign key relationship in Diesel schemas.

use crate::traits::TypedColumn;

/// Defines a trait for determining whether two types are compatible
/// for a foreign key relationship in Diesel.
pub trait ForeignKeyCompatibleSqlType<Other> {}

impl<T> ForeignKeyCompatibleSqlType<T> for T {}
impl<T> ForeignKeyCompatibleSqlType<diesel::sql_types::Nullable<T>> for T {}
impl<T> ForeignKeyCompatibleSqlType<T> for diesel::sql_types::Nullable<T> {}

/// Defines a trait for determining whether two types are compatible
/// for a foreign key relationship in Diesel.
pub trait ForeignKeyCompatibleType<Other>: Into<Other> {}

impl<T> ForeignKeyCompatibleType<T> for T {}
impl<T> ForeignKeyCompatibleType<Option<T>> for T {}

/// Helper trait for a single compatible type.
pub trait ForeignKeyCompatibleColumn<Other: TypedColumn>: TypedColumn {}

impl<C1, C2> ForeignKeyCompatibleColumn<C2> for C1
where
    C1: TypedColumn,
    C2: TypedColumn,
    C1::SqlType: ForeignKeyCompatibleSqlType<C2::SqlType>,
    C1::Type: ForeignKeyCompatibleType<C2::Type>,
{
}

/// Helper trait for tuples of compatible types.
pub trait ForeignKeyCompatibleColumns<Other> {}

impl<C2: TypedColumn, C1: TypedColumn> ForeignKeyCompatibleColumns<(C2,)> for (C1,) where
    C1: ForeignKeyCompatibleColumn<C2>
{
}

impl<C1: TypedColumn, C2: TypedColumn, C3: TypedColumn, C4: TypedColumn>
    ForeignKeyCompatibleColumns<(C1, C2)> for (C3, C4)
where
    C3: ForeignKeyCompatibleColumn<C1>,
    C4: ForeignKeyCompatibleColumn<C2>,
{
}

impl<
    C1: TypedColumn,
    C2: TypedColumn,
    C3: TypedColumn,
    C4: TypedColumn,
    C5: TypedColumn,
    C6: TypedColumn,
> ForeignKeyCompatibleColumns<(C1, C2, C3)> for (C4, C5, C6)
where
    C4: ForeignKeyCompatibleColumn<C1>,
    C5: ForeignKeyCompatibleColumn<C2>,
    C6: ForeignKeyCompatibleColumn<C3>,
{
}
