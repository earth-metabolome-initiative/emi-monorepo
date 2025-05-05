//! Submodule providing the enumeration for the internal messages used
//! in the DB/WebSocket worker.

use futures::channel::mpsc::TrySendError;
use gloo::net::websocket::WebSocketError;
use ws_messages::{B2FMessage, DBMessage, F2BMessage, frontend::Subscription};

use crate::errors::{db_errors::DBError, ws_errors::WSError};

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
    /// Message indicating an error related to the WebSocket.
    WSError(WSError),
}

impl From<Result<sqlite_wasm_rs::export::OpfsSAHPoolUtil, sqlite_wasm_rs::export::OpfsSAHError>>
    for InternalMessage
{
    fn from(
        value: Result<
            sqlite_wasm_rs::export::OpfsSAHPoolUtil,
            sqlite_wasm_rs::export::OpfsSAHError,
        >,
    ) -> Self {
        match value {
            Ok(_) => Self::DB(db_internal_message::DBInternalMessage::Connect),
            Err(err) => Self::DBError(err.into()),
        }
    }
}

impl From<diesel::result::Error> for InternalMessage {
    fn from(value: diesel::result::Error) -> Self {
        Self::DBError(value.into())
    }
}

impl From<diesel::ConnectionError> for InternalMessage {
    fn from(value: diesel::ConnectionError) -> Self {
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

impl From<WSError> for InternalMessage {
    fn from(value: WSError) -> Self {
        Self::WSError(value)
    }
}

impl From<DBMessage> for InternalMessage {
    fn from(value: DBMessage) -> Self {
        Self::WS(value.into())
    }
}

impl From<WebSocketError> for InternalMessage {
    fn from(value: WebSocketError) -> Self {
        Self::WSError(value.into())
    }
}

impl From<bincode::error::EncodeError> for InternalMessage {
    fn from(value: bincode::error::EncodeError) -> Self {
        Self::WSError(value.into())
    }
}

impl From<bincode::error::DecodeError> for InternalMessage {
    fn from(value: bincode::error::DecodeError) -> Self {
        Self::WSError(value.into())
    }
}

impl From<B2FMessage> for InternalMessage {
    fn from(value: B2FMessage) -> Self {
        Self::WS(value.into())
    }
}

impl From<F2BMessage> for InternalMessage {
    fn from(value: F2BMessage) -> Self {
        Self::WS(value.into())
    }
}

impl From<Subscription> for InternalMessage {
    fn from(value: Subscription) -> Self {
        Self::WS(value.into())
    }
}

impl From<TrySendError<F2BMessage>> for InternalMessage {
    fn from(value: TrySendError<F2BMessage>) -> Self {
        Self::WSError(value.into())
    }
}
