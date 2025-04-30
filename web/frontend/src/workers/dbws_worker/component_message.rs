//! Submodule defining the messages from the component to the web-worker.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::{CrudPrimaryKeyOperation, CrudTableOperation};

use super::C2DBMessage;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
/// Messages from the component to the web-worker.
pub enum ComponentMessage {
    /// Message from the component to the database.
    DB(C2DBMessage),
}

impl From<C2DBMessage> for ComponentMessage {
    fn from(msg: C2DBMessage) -> Self {
        ComponentMessage::DB(msg)
    }
}

impl From<CrudPrimaryKeyOperation<Row>> for ComponentMessage {
    fn from(msg: CrudPrimaryKeyOperation<Row>) -> Self {
        Self::DB(msg.into())
    }
}

impl From<CrudTableOperation<Rows>> for ComponentMessage {
    fn from(msg: CrudTableOperation<Rows>) -> Self {
        Self::DB(msg.into())
    }
}
