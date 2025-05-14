//! Submodule providing the enumeration of internal messages used
//! for the DB portion of the DB/WebSocket worker.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::CRUD;
use ws_messages::DBMessage;
use yew_agent::worker::HandlerId;

use crate::workers::dbws_worker::C2DBMessage;

#[derive(Debug, Clone)]
/// Enumeration of internal messages used for the DB portion of the DB/WebSocket
/// worker.
pub enum DBInternalMessage {
    /// Indicates that a new connection to the database has been requested.
    Connect,
    /// Indicates that a message from the frontend should be attempted
    /// to be processed once again.
    Retry((C2DBMessage, HandlerId)),
    /// Message from the backend database involving several rows.
    DB(DBMessage),
}

impl From<(Rows, CRUD)> for DBInternalMessage {
    fn from(msg: (Rows, CRUD)) -> Self {
        DBInternalMessage::DB(msg.into())
    }
}

impl From<(Row, CRUD)> for DBInternalMessage {
    fn from(msg: (Row, CRUD)) -> Self {
        DBInternalMessage::DB(msg.into())
    }
}

impl From<DBMessage> for DBInternalMessage {
    fn from(msg: DBMessage) -> Self {
        DBInternalMessage::DB(msg)
    }
}
