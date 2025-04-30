//! Submodule providing the enumeration of internal messages used
//! for the DB portion of the DB/WebSocket worker.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::CRUD;

/// Enumeration of internal messages used for the DB portion of the DB/WebSocket
/// worker.
pub enum DBInternalMessage {
    /// Indicates that a new connection to the database has been requested.
    Connect,
    /// Message from the backend database involving several rows.
    Rows(Rows, CRUD),
    /// Message from the backend database involving a single row.
    Row(Row, CRUD),
}

impl From<(Rows, CRUD)> for DBInternalMessage {
    fn from(msg: (Rows, CRUD)) -> Self {
        DBInternalMessage::Rows(msg.0, msg.1)
    }
}

impl From<(Row, CRUD)> for DBInternalMessage {
    fn from(msg: (Row, CRUD)) -> Self {
        DBInternalMessage::Row(msg.0, msg.1)
    }
}
