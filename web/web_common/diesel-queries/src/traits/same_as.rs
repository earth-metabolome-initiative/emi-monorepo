//! Handles same-as updates for Diesel model builders.

use diesel::Table;

use crate::traits::{ForeignKey, ForeignKeyCompatibleColumn, TableIsExtensionOf, TypedColumn};

/// Trait defining the existance of an horizontal same-as relationship.
pub trait HorizontalSameAs<Referenced, Key>: TypedColumn {}

impl<HostColumn, Referenced, Key> HorizontalSameAs<Referenced, Key> for HostColumn
where
    HostColumn: TypedColumn,
    Referenced: TypedColumn,
    Key: ForeignKeyCompatibleColumn<<Referenced::Table as Table>::PrimaryKey>,
    HostColumn: ForeignKeyCompatibleColumn<Referenced>,
    <Referenced::Table as Table>::PrimaryKey: TypedColumn<Table = Referenced::Table>,
    HostColumn::Table:
        ForeignKey<(Key, HostColumn), (<Referenced::Table as Table>::PrimaryKey, Referenced)>,
{
}

/// Trait defining the existance of a vertical same-as relationship.
pub trait VerticalSameAs<Ancestral: TypedColumn>: TypedColumn
where
    Self::Table: TableIsExtensionOf<Ancestral::Table>,
    Ancestral: ForeignKeyCompatibleColumn<Self>,
{
}
