//! Submodule providing the enumeration of the websocket messages from the
//! DB to either the frontend or backend. It may be used for both use cases.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::CRUD;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Message from the DB to either the frontend or backend.
pub enum DBMessage {
    /// A reply to a request involving several rows.
    Rows(Rows, CRUD),
    /// A row-wise operation result.
    Row(Row, CRUD),
}

impl From<(Rows, CRUD)> for DBMessage {
    fn from((rows, crud): (Rows, CRUD)) -> Self {
        DBMessage::Rows(rows, crud)
    }
}

impl From<(Row, CRUD)> for DBMessage {
    fn from((row, crud): (Row, CRUD)) -> Self {
        DBMessage::Row(row, crud)
    }
}

