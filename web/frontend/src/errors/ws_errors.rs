//! This module defines the `WSError` enum, which represents various errors
//! that can occur when interacting with the WebSocket in the frontend.

use std::fmt::Display;

use futures::channel::mpsc::{SendError, TrySendError};
use gloo::net::websocket::WebSocketError;
use ws_messages::F2BMessage;

#[derive(Debug, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
/// An enum representing various device errors.
pub enum WSError {
    /// When the connection to the web socket fails.
    Connection,
    /// The `close` event
    ConnectionClose,
    /// When the message cannot be serialized.
    SerializationError,
    /// When the message cannot be deserialized.
    DeserializationError,
}

impl From<WebSocketError> for WSError {
    fn from(err: WebSocketError) -> Self {
        match err {
            WebSocketError::ConnectionError => WSError::Connection,
            WebSocketError::ConnectionClose(_close_event) => WSError::ConnectionClose,
            WebSocketError::MessageSendError(js_error) => {
                unimplemented!("MessageSendError: {:?}", js_error)
            }
            err => {
                unimplemented!("WebSocketError: {err:?}")
            }
        }
    }
}

impl From<SendError> for WSError {
    fn from(err: SendError) -> Self {
        if err.is_full() { WSError::Connection } else { WSError::ConnectionClose }
    }
}

impl From<TrySendError<F2BMessage>> for WSError {
    fn from(err: TrySendError<F2BMessage>) -> Self {
        err.into_send_error().into()
    }
}

impl From<bincode::error::EncodeError> for WSError {
    fn from(_err: bincode::error::EncodeError) -> Self {
        WSError::SerializationError
    }
}

impl From<bincode::error::DecodeError> for WSError {
    fn from(_err: bincode::error::DecodeError) -> Self {
        WSError::DeserializationError
    }
}

impl Display for WSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WSError::Connection => write!(f, "Failed to connect to the backend."),
            WSError::ConnectionClose => {
                write!(f, "Connection closed by the backend.")
            }
            WSError::SerializationError => {
                write!(f, "Failed to serialize the message.")
            }
            WSError::DeserializationError => {
                write!(f, "Failed to deserialize the message.")
            }
        }
    }
}
