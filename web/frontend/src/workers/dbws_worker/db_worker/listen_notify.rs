//! Submodule providing the Listen/Notify emulation for the `SQLite` database.
//!
//! We register a listener component for the database depending on the
//! `TableName` and `TablePrimaryKey` that have been queries for by the
//! component. Whenever a change is made to either the table or the specific
//! row, the listener will be notified.

use std::collections::HashMap;

use core_structures::tables::{table_names::TableName, table_primary_keys::TablePrimaryKey};
use web_common_traits::{
    crud::CRUD,
    prelude::{Row, Rows, Tabular},
};
use ws_messages::DBMessage;
use yew_agent::worker::HandlerId;

use crate::workers::DBWSWorker;

#[derive(Debug, Clone, Default)]
/// Struct handling the Listen/Notify emulation for the `SQLite` database
/// and the propagation of messages to the Yew components.
pub struct ListenNotify {
    table_listeners: HashMap<TableName, Vec<HandlerId>>,
    row_listeners: HashMap<TablePrimaryKey, Vec<HandlerId>>,
}

impl ListenNotify {
    pub(crate) fn register_table_listener(
        &mut self,
        subscriber_id: HandlerId,
        table_name: TableName,
    ) {
        let table_listeners = self.table_listeners.entry(table_name).or_default();
        if !table_listeners.contains(&subscriber_id) {
            table_listeners.push(subscriber_id);
        }
    }

    pub(crate) fn register_row_listener(
        &mut self,
        subscriber_id: HandlerId,
        table_primary_key: TablePrimaryKey,
    ) {
        let row_listeners = self.row_listeners.entry(table_primary_key).or_default();
        if !row_listeners.contains(&subscriber_id) {
            row_listeners.push(subscriber_id);
        }
    }

    /// Removes the provided table listener from the list of listeners and
    /// returns the list of orphaned tables which have no listeners anymore.
    pub(crate) fn remove_table_listener(&mut self, subscriber_id: HandlerId) -> Vec<TableName> {
        let mut orphaned_tables = Vec::new();
        for (table_name, listeners) in &mut self.table_listeners {
            if let Some(pos) = listeners.iter().position(|id| *id == subscriber_id) {
                listeners.remove(pos);
                if listeners.is_empty() {
                    orphaned_tables.push(*table_name);
                }
            }
        }
        orphaned_tables
    }

    /// Removes the provided row listener from the list of listeners and
    /// returns the list of orphaned rows which have no listeners anymore.
    pub(crate) fn remove_row_listener(&mut self, subscriber_id: HandlerId) -> Vec<TablePrimaryKey> {
        let mut orphaned_rows = Vec::new();
        for (table_primary_key, listeners) in &mut self.row_listeners {
            if let Some(pos) = listeners.iter().position(|id| *id == subscriber_id) {
                listeners.remove(pos);
                if listeners.is_empty() {
                    orphaned_rows.push(*table_primary_key);
                }
            }
        }
        orphaned_rows
    }

    /// Notifies the listeners of the provided table that a change has occurred.
    pub fn notify_table_listeners<T>(
        &self,
        tabular: &T,
        crud: CRUD,
        scope: &yew_agent::prelude::WorkerScope<DBWSWorker>,
    ) where
        T: Tabular<TableName = TableName> + Clone,
        DBMessage: From<(T, CRUD)>,
    {
        let table_name = tabular.table_name();
        if let Some(listeners) = self.table_listeners.get(&table_name) {
            for listener in listeners {
                // Notify the listener of the table change
                scope.respond(*listener, (tabular.clone(), crud).into());
            }
        }
    }

    /// Notifies the listeners for the provided rows.
    pub fn notify_rows_listeners<R>(
        &self,
        rows: &R,
        crud: CRUD,
        scope: &yew_agent::prelude::WorkerScope<DBWSWorker>,
    ) where
        R: Rows<TableName = TableName, PrimaryKey = TablePrimaryKey> + Clone,
        DBMessage: From<(R, CRUD)>,
    {
        self.notify_table_listeners(rows, crud, scope);

        for primary_key in rows.primary_keys() {
            if let Some(listeners) = self.row_listeners.get(&primary_key) {
                for listener in listeners {
                    // Notify the listener of the row change
                    scope.respond(*listener, (rows.clone(), crud).into());
                }
            }
        }
    }

    /// Notifies the listeners for the provided row.
    pub fn notify_row_listeners<R>(
        &self,
        row: &R,
        crud: CRUD,
        scope: &yew_agent::prelude::WorkerScope<DBWSWorker>,
    ) where
        R: Row<TableName = TableName, PrimaryKey = TablePrimaryKey> + Clone,
        DBMessage: From<(R, CRUD)>,
    {
        self.notify_table_listeners(row, crud, scope);

        if let Some(listeners) = self.row_listeners.get(&row.primary_key()) {
            for listener in listeners {
                // Notify the listener of the row change
                scope.respond(*listener, (row.clone(), crud).into());
            }
        }
    }
}
