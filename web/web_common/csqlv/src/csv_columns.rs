//! Submodule providing the CSV Column struct.

use crate::{
    CSVSchemaError, csv_table::CSVTable, data_types::DataType, metadata::CSVColumnMetadata,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// Struct representing a CSV column.
pub struct CSVColumn<'a> {
    pub(crate) table: &'a CSVTable<'a>,
    pub(crate) column_metadata: &'a CSVColumnMetadata,
}

impl<'a> CSVColumn<'a> {
    /// Returns the name of the column.
    pub fn name(&self) -> Result<String, CSVSchemaError> {
        self.column_metadata.name(self.table.schema)
    }

    #[must_use]
    /// Returns the data type of the column.
    pub fn data_type(&self) -> &DataType {
        &self.column_metadata.data_type
    }

    #[must_use]
    /// Returns whether this column is artificial.
    pub fn is_artificial(&self) -> bool {
        self.column_metadata.artificial
    }

    #[must_use]
    /// Returns whether this column is the primary key.
    pub fn is_primary_key(&self) -> bool {
        self.column_metadata.primary_key
    }

    #[must_use]
    /// Returns whether this column is UNIQUE.
    pub fn is_unique(&self) -> bool {
        self.column_metadata.unique
    }

    #[must_use]
    /// Returns whether this column is NULLABLE.
    pub fn is_nullable(&self) -> bool {
        self.column_metadata.nullable
    }

    #[must_use]
    /// Returns the foreign key table, if any.
    ///
    /// # Panics
    /// * If the schema is in an invalid state and the foreign table does not
    ///   exist.
    pub fn foreign_table(&self) -> Option<CSVTable<'_>> {
        self.column_metadata.foreign_table_name.as_ref().map(|foreign_table_name| {
            self.table.schema.table_from_name(foreign_table_name).unwrap()
        })
    }

    #[must_use]
    /// Returns the foreign key column, if any.
    pub fn foreign_column_name(&self) -> Option<&str> {
        self.column_metadata.foreign_column_name.as_deref()
    }
}
