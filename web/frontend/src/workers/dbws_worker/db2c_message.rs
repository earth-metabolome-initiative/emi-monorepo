//! Submodule providing the enumeration of messages from the `SQLite` database
//! web-worker to the frontend Yew components.

use core_structures::tables::{row::Row, rows::Rows};
use serde::{Deserialize, Serialize};
use web_common_traits::crud::CRUD;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the `SQLite` database web-worker to the frontend.
pub enum DB2CMessage {
    /// Message from the backend database involving several rows.
    Rows(Rows, CRUD),
    /// Message from the backend database involving a single row.
    Row(Row, CRUD),
}

impl From<(Rows, CRUD)> for DB2CMessage {
    fn from(msg: (Rows, CRUD)) -> Self {
        DB2CMessage::Rows(msg.0, msg.1)
    }
}

impl From<(Row, CRUD)> for DB2CMessage {
    fn from(msg: (Row, CRUD)) -> Self {
        DB2CMessage::Row(msg.0, msg.1)
    }
}
