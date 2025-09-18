//! Submodule defining the `ColumnLike` trait and its implementations.

use sqlparser::ast::{ColumnOption, DataType};

/// Trait representing entities that are "column-like", such as table columns.
pub trait ColumnLike {
    /// Returns the data type of the column-like entity, if it has one.
    fn data_type(&self) -> &DataType;

    /// Returns whether the column-like entity has a data type of `UUID`.
    fn is_uuid_type(&self) -> bool {
        self.data_type() == &DataType::Uuid
    }

    /// Returns whether the column is a primary key.
    fn is_primary_key(&self) -> bool;
}

impl ColumnLike for sqlparser::ast::ColumnDef {
    fn data_type(&self) -> &DataType {
        &self.data_type
    }

    fn is_primary_key(&self) -> bool {
        self.options
            .iter()
            .any(|opt| matches!(opt.option, ColumnOption::Unique { is_primary: true, .. }))
    }
}
