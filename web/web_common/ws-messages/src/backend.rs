//! Submodule providing the enumeration of the websocket messages from the
//! backend to the frontend.

use core_structures::tables::read::B2FReadAll;

/// Websocket messages from the backend to the frontend.
pub enum B2FMessage {
    /// Health check message reply.
    Pong,
    /// A `ReadAll` message reply from the backend.
    B2FReadAll(B2FReadAll),
}

impl From<B2FReadAll> for B2FMessage {
    fn from(msg: B2FReadAll) -> Self {
        B2FMessage::B2FReadAll(msg)
    }
}

impl<T> From<Vec<T>> for B2FMessage
where
    Vec<T>: Into<B2FReadAll>,
{
    fn from(msg: Vec<T>) -> Self {
        B2FMessage::B2FReadAll(msg.into())
    }
}
