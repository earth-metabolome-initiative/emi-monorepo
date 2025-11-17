//! Submodule defining the `Columns` trait for Diesel queries.

use crate::traits::TypedColumn;

/// Trait for types that have associated columns in Diesel.
pub trait Columns {
    /// Table of the columns.
    /// TODO! replace this by using the `diesel::HasTable` supertrait when PR is
    /// merged.
    type Table: diesel::Table;

    /// Ennuple defining the types of the columns.
    type Type;
}

impl<C1> Columns for (C1,)
where
    C1: TypedColumn,
{
    type Table = C1::Table;
    type Type = (C1::Type,);
}

impl<C1, C2> Columns for (C1, C2)
where
    C1: TypedColumn,
    C2: TypedColumn<Table = C1::Table>,
{
    type Table = C1::Table;
    type Type = (C1::Type, C2::Type);
}
