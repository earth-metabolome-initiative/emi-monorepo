//! Handles same-as updates for Diesel model builders.

use crate::traits::TableIsExtensionOf;

/// Trait defining the existance of an horizontal same-as relationship.
pub trait HorizontalSameAs<
    Referenced: diesel::Column,
	// The key in the current table that references the Referenced column
    Key: diesel::Column<Table = Self::Table>,
>: diesel::Column
{
}

/// Trait defining the existance of a vertical same-as relationship.
pub trait VerticalSameAs<Referenced: diesel::Column>: diesel::Column
where
    Self::Table: TableIsExtensionOf<Referenced::Table>,
{
}
