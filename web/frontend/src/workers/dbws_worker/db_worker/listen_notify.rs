//! Submodule providing the Listen/Notify emulation for the `SQLite` database.
//!
//! We register a listener component for the database depending on the
//! `TableName` and `TablePrimaryKey` that have been queries for by the
//! component. Whenever a change is made to either the table or the specific
//! row, the listener will be notified.

use std::collections::HashMap;

use core_structures::tables::{table_names::TableName, table_primary_keys::TablePrimaryKey};
use yew_agent::worker::HandlerId;

#[derive(Debug, Clone, Default)]
/// Struct handling the Listen/Notify emulation for the `SQLite` database
/// and the propagation of messages to the Yew components.
pub struct ListenNotify {
    table_listeners: HashMap<TableName, Vec<HandlerId>>,
    row_listeners: HashMap<TablePrimaryKey, Vec<HandlerId>>,
}
