//! This module defines the `DBError` enum, which represents various errors
//! that can occur when interacting with the database in a web application.

use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
/// An enum representing various device errors.
pub enum DBError {
    /// When the connection to the database fails.
    Connection,
    /// When installing `SAHPool` fails.
    InstallSAHPool,
    /// When installing `RelaxedIdb` fails.
    InstallRelaxedIdb,
    /// When a query fails.
    QueryFailed,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::Connection => write!(f, "Failed to connect to the database."),
            DBError::QueryFailed => write!(f, "Query failed."),
            DBError::InstallSAHPool => write!(f, "Failed to install SAHPool."),
            DBError::InstallRelaxedIdb => write!(f, "Failed to install RelaxedIdb."),
        }
    }
}

impl From<sqlite_wasm_rs::export::OpfsSAHError> for DBError {
    fn from(_err: sqlite_wasm_rs::export::OpfsSAHError) -> Self {
        DBError::InstallSAHPool
    }
}

impl From<sqlite_wasm_rs::relaxed_idb_vfs::RelaxedIdbError> for DBError {
    fn from(_err: sqlite_wasm_rs::relaxed_idb_vfs::RelaxedIdbError) -> Self {
        DBError::InstallRelaxedIdb
    }
}

impl From<diesel::result::Error> for DBError {
    fn from(_err: diesel::result::Error) -> Self {
        DBError::QueryFailed
    }
}

impl From<diesel::ConnectionError> for DBError {
    fn from(_err: diesel::ConnectionError) -> Self {
        DBError::Connection
    }
}
