//! Submodule providing the enumeration of internal messages used
//! for the WS portion of the DB/WebSocket worker.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::{CrudPrimaryKeyOperation, CrudTableOperation};
use ws_messages::{B2FMessage, F2BMessage, frontend::Unsubscribe};

use crate::workers::dbws_worker::C2DBMessage;

/// Enumeration of internal messages used for the WS portion of the DB/WebSocket
/// worker.
pub enum WSInternalMessage {
    /// Request to attempt a new connection to the backend.
    Connect,
    /// Message from the backend to the frontend.
    B2F(B2FMessage),
    /// Message from the frontend to the backend.
    F2B(F2BMessage),
}

impl From<B2FMessage> for WSInternalMessage {
    fn from(value: B2FMessage) -> Self {
        Self::B2F(value)
    }
}

impl From<F2BMessage> for WSInternalMessage {
    fn from(value: F2BMessage) -> Self {
        Self::F2B(value)
    }
}

impl From<CrudPrimaryKeyOperation<Row>> for WSInternalMessage {
    fn from(msg: CrudPrimaryKeyOperation<Row>) -> Self {
        Self::F2B(msg.into())
    }
}

impl From<CrudTableOperation<Rows>> for WSInternalMessage {
    fn from(msg: CrudTableOperation<Rows>) -> Self {
        Self::F2B(msg.into())
    }
}

impl From<Unsubscribe> for WSInternalMessage {
    fn from(value: Unsubscribe) -> Self {
        Self::F2B(value.into())
    }
}

impl From<C2DBMessage> for WSInternalMessage {
    fn from(value: C2DBMessage) -> Self {
        match value {
            C2DBMessage::Row(row) => Self::F2B(row.into()),
            C2DBMessage::Table(table) => Self::F2B(table.into()),
        }
    }
}
