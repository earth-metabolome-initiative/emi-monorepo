//! Submodule providing the CSV Column struct.

use crate::{csv_table::CSVTable, data_types::DataType, metadata::CSVColumnMetadata};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a CSV column.
pub struct CSVColumn<'a> {
    pub(crate) table: &'a CSVTable<'a>,
    pub(crate) column_metadata: &'a CSVColumnMetadata,
}

impl<'a> CSVColumn<'a> {
    /// Returns the name of the column.
    pub fn name(&self) -> &str {
        &self.column_metadata.name
    }

    /// Returns the data type of the column.
    pub fn data_type(&self) -> &DataType {
        &self.column_metadata.data_type
    }

    /// Returns whether this column is artificial.
    pub fn is_artificial(&self) -> bool {
        self.column_metadata.artificial
    }

    /// Returns whether this column is the primary key.
    pub fn is_primary_key(&self) -> bool {
        self.column_metadata.primary_key
    }

    /// Returns whether this column is UNIQUE.
    pub fn is_unique(&self) -> bool {
        self.column_metadata.unique
    }

    /// Returns whether this column is NULLABLE.
    pub fn is_nullable(&self) -> bool {
        self.column_metadata.nullable
    }

    /// Returns the foreign key table, if any.
    pub fn foreign_table(&self) -> Option<CSVTable<'_>> {
        self.column_metadata
            .foreign_table_name
            .as_ref()
            .map(|foreign_table_name| {
                self.table
                    .schema
                    .table_from_name(foreign_table_name)
                    .unwrap()
            })
    }

    /// Returns the foreign key column, if any.
    pub fn foreign_column_name(&self) -> Option<&str> {
        self.column_metadata.foreign_column_name.as_deref()
    }
}
