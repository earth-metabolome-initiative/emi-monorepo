//! Submodule defining a message enumeration regarding the
//! unsubscription of a table or row.

use core_structures::tables::{table_names::TableName, table_primary_keys::TablePrimaryKey};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Enumeration of messages regarding the unsubscription of a table or row.
pub enum Unsubscribe {
    /// Unsubscribe from a table.
    Table(TableName),
    /// Unsubscribe from a row.
    Row(TablePrimaryKey),
}

impl From<TableName> for Unsubscribe {
    fn from(table: TableName) -> Self {
        Unsubscribe::Table(table)
    }
}

impl From<TablePrimaryKey> for Unsubscribe {
    fn from(row: TablePrimaryKey) -> Self {
        Unsubscribe::Row(row)
    }
}
