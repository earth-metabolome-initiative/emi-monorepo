//! Submodule providing the enumeration for the internal messages used
//! in the DB/WebSocket worker.

use sqlite_wasm_rs::export::{OpfsSAHError, OpfsSAHPoolUtil};

use crate::errors::db_errors::DBError;

pub mod db_internal_message;
pub mod ws_internal_message;

/// Enumeration of internal messages used in the DB/WebSocket worker.
pub enum InternalMessage {
    /// Message related to the database operations.
    DB(db_internal_message::DBInternalMessage),
    /// Message related to the WebSocket operations.
    WS(ws_internal_message::WSInternalMessage),
    /// Message indicating an error related to the database.
    DBError(DBError),
}

impl From<Result<OpfsSAHPoolUtil, OpfsSAHError>> for InternalMessage {
    fn from(value: Result<OpfsSAHPoolUtil, OpfsSAHError>) -> Self {
        match value {
            Ok(_) => Self::DB(db_internal_message::DBInternalMessage::SAHPoolInstalled),
            Err(err) => Self::DBError(err.into()),
        }
    }
}

impl From<diesel::result::Error> for InternalMessage {
    fn from(value: diesel::result::Error) -> Self {
        Self::DBError(value.into())
    }
}

impl From<db_internal_message::DBInternalMessage> for InternalMessage {
    fn from(value: db_internal_message::DBInternalMessage) -> Self {
        Self::DB(value)
    }
}

impl From<ws_internal_message::WSInternalMessage> for InternalMessage {
    fn from(value: ws_internal_message::WSInternalMessage) -> Self {
        Self::WS(value)
    }
}
