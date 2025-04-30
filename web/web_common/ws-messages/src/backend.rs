//! Submodule providing the enumeration of the websocket messages from the
//! backend to the frontend.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::CRUD;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Websocket messages from the backend to the frontend.
pub enum B2FMessage {
    /// Health check message reply.
    Pong,
    /// A a reply to a request involving several rows.
    Rows(Rows, CRUD),
    /// A row-wise operation result.
    Row(Row, CRUD),
}

impl From<(Rows, CRUD)> for B2FMessage {
    fn from(msg: (Rows, CRUD)) -> Self {
        B2FMessage::Rows(msg.0, msg.1)
    }
}

impl From<(Row, CRUD)> for B2FMessage {
    fn from(msg: (Row, CRUD)) -> Self {
        B2FMessage::Row(msg.0, msg.1)
    }
}

impl TryFrom<gloo::net::websocket::Message> for B2FMessage {
    type Error = bincode::error::DecodeError;

    fn try_from(msg: gloo::net::websocket::Message) -> Result<Self, Self::Error> {
        match msg {
            gloo::net::websocket::Message::Text(text) => {
                unreachable!("Text messages are not supported: {text}")
            }
            gloo::net::websocket::Message::Bytes(bytes) => {
                let (message, number_of_bytes) =
                    bincode::serde::decode_from_slice(&bytes, bincode::config::standard())?;
                assert_eq!(
                    number_of_bytes,
                    bytes.len(),
                    "Number of bytes read does not match the length of the message"
                );
                Ok(message)
            }
        }
    }
}
