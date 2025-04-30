//! Submodule providing the enumeration of messages from a component to the
//! `SQLite` database web-worker.

use core_structures::tables::{row::Row, rows::Rows};
use serde::{Deserialize, Serialize};
use web_common_traits::crud::{CrudPrimaryKeyOperation, CrudTableOperation};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Messages from the frontend to the web-worker.
pub enum C2DBMessage {
    /// A row-wise operation on a table.
    Row(CrudPrimaryKeyOperation<Row>),
    /// A table-wise operation.
    Table(CrudTableOperation<Rows>),
}

impl From<CrudPrimaryKeyOperation<Row>> for C2DBMessage {
    fn from(msg: CrudPrimaryKeyOperation<Row>) -> Self {
        C2DBMessage::Row(msg)
    }
}

impl From<CrudTableOperation<Rows>> for C2DBMessage {
    fn from(msg: CrudTableOperation<Rows>) -> Self {
        C2DBMessage::Table(msg)
    }
}
