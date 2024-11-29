//! Submodule providing the CSV Column struct.

use crate::{csv_table::CSVTable, metadata::CSVColumnMetadata};

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
}