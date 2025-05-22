//! Submodule providing the enumeration for the possible subscription messages.

use core_structures::tables::{table_names::TableName, table_primary_keys::TablePrimaryKey};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Enumeration of the possible subscription messages.
pub enum Subscription {
    /// Subscribe to a specific row.
    Row(TablePrimaryKey),
    /// Subscribe to a specific table.
    Table(TableName),
}

impl From<TablePrimaryKey> for Subscription {
    fn from(table_primary_key: TablePrimaryKey) -> Self {
        Subscription::Row(table_primary_key)
    }
}

impl From<TableName> for Subscription {
    fn from(table_name: TableName) -> Self {
        Subscription::Table(table_name)
    }
}
