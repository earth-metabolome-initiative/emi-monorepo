//! Handles same-as updates for Diesel model builders.

use crate::traits::{TableIsExtensionOf, TypedColumn};

/// Trait defining the existance of an horizontal same-as relationship.
pub trait HorizontalSameAs<
    Referenced: TypedColumn<SqlType = Self::SqlType, Type = Self::Type>,
	// The key in the current table that references the Referenced column
    Key: TypedColumn<Table = Self::Table>,
>: TypedColumn
{
}

/// Trait defining the existance of a vertical same-as relationship.
pub trait VerticalSameAs<Ancestral: TypedColumn<SqlType = Self::SqlType, Type = Self::Type>>:
    TypedColumn
where
    Self::Table: TableIsExtensionOf<Ancestral::Table>,
{
}
