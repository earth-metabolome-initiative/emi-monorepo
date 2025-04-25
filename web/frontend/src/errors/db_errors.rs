//! This module defines the `DBError` enum, which represents various errors
//! that can occur when interacting with the database in a web application.

use std::fmt::Display;

use sqlite_wasm_rs::export::OpfsSAHError;

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
/// An enum representing various device errors.
pub enum DBError {
    /// When the connection to the database fails.
    Connection,
    /// When installing `SAHPool` fails.
    InstallSAHPool,
    /// When a query fails.
    QueryFailed,
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::Connection => write!(f, "Failed to connect to the database."),
            DBError::QueryFailed => write!(f, "Query failed."),
            DBError::InstallSAHPool => write!(f, "Failed to install SAHPool."),
        }
    }
}

impl From<OpfsSAHError> for DBError {
    fn from(_err: OpfsSAHError) -> Self {
        DBError::InstallSAHPool
    }
}

impl From<diesel::result::Error> for DBError {
    fn from(_err: diesel::result::Error) -> Self {
        DBError::QueryFailed
    }
}
