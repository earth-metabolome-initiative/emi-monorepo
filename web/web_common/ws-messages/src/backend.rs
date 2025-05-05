//! Submodule providing the enumeration of the websocket messages from the
//! backend to the frontend.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::CRUD;

use crate::DBMessage;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Websocket messages from the backend to the frontend.
pub enum B2FMessage {
    /// A reply to a request involving the DB.
    DB(DBMessage),
}

impl From<(Rows, CRUD)> for B2FMessage {
    fn from(msg: (Rows, CRUD)) -> Self {
        B2FMessage::DB(msg.into())
    }
}

impl From<(Row, CRUD)> for B2FMessage {
    fn from(msg: (Row, CRUD)) -> Self {
        B2FMessage::DB(msg.into())
    }
}

impl From<DBMessage> for B2FMessage {
    fn from(msg: DBMessage) -> Self {
        B2FMessage::DB(msg)
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

#[cfg(feature = "backend")]
impl From<B2FMessage> for actix_web::web::Bytes {
    fn from(message: B2FMessage) -> Self {
        actix_web::web::Bytes::from(
            bincode::serde::encode_to_vec(&message, bincode::config::standard()).unwrap(),
        )
    }
}
