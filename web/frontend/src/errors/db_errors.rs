//! This module defines the `DBError` enum, which represents various errors
//! that can occur when interacting with the database in a web application.

use std::fmt::Display;

#[derive(Debug, PartialEq)]
/// An enum representing various device errors.
pub enum DBError {
    /// When the connection to the database fails.
    Connection,
    /// When installing `SAHPool` fails.
    InstallSAHPool,
    /// When a query fails.
    QueryFailed(diesel::result::Error),
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::Connection => write!(f, "Failed to connect to the database."),
            DBError::QueryFailed(err) => write!(f, "Query failed: {err}"),
            DBError::InstallSAHPool => write!(f, "Failed to install SAHPool."),
        }
    }
}

impl From<sqlite_wasm_rs::sahpool_vfs::OpfsSAHError> for DBError {
    fn from(_err: sqlite_wasm_rs::sahpool_vfs::OpfsSAHError) -> Self {
        DBError::InstallSAHPool
    }
}

impl From<diesel::result::Error> for DBError {
    fn from(err: diesel::result::Error) -> Self {
        DBError::QueryFailed(err)
    }
}

impl From<diesel::ConnectionError> for DBError {
    fn from(_err: diesel::ConnectionError) -> Self {
        DBError::Connection
    }
}
