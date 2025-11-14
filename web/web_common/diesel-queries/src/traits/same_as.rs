//! Handles same-as updates for Diesel model builders.

use crate::traits::{GetColumn, SetColumn, extension_of::TableIsExtensionOf};

/// Trait defining the existance of an horizontal same-as relationship.
pub trait HasHorizontalSameAs<Referenced: diesel::Column, Key: diesel::Column>:
    diesel::Column
{
}

/// Trait for handling same-as updates between Diesel model builders.
pub trait IsVerticallySameAsHelper<Referenced: diesel::Column>: diesel::Column {}

impl<Referenced: diesel::Column, T: diesel::Column> IsVerticallySameAsHelper<Referenced> for T where
    T::Table: TableIsExtensionOf<<Referenced as diesel::Column>::Table>
{
}

/// Trait defining the existance of a vertical same-as relationship.
pub trait IsVerticallySameAs<Referenced: diesel::Column>: diesel::Column {}
