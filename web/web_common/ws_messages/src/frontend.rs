//! Submodule providing the enumeration of the websocket messages from the
//! frontend to the backend.

use core_structures::tables::{row::Row, rows::Rows};
use web_common_traits::crud::{CrudPrimaryKeyOperation, CrudTableOperation};

pub use crate::Subscription;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
/// Websocket messages from the frontend to the backend.
pub enum F2BMessage {
    /// A row-wise operation on a table.
    Row(CrudPrimaryKeyOperation<Row>),
    /// A table-wise operation.
    Table(CrudTableOperation<Rows>),
    /// Request to stop submitting messages.
    Unsubscribe(Subscription),
}

impl From<CrudPrimaryKeyOperation<Row>> for F2BMessage {
    fn from(msg: CrudPrimaryKeyOperation<Row>) -> Self {
        F2BMessage::Row(msg)
    }
}

impl From<CrudTableOperation<Rows>> for F2BMessage {
    fn from(msg: CrudTableOperation<Rows>) -> Self {
        F2BMessage::Table(msg)
    }
}

impl From<Subscription> for F2BMessage {
    fn from(msg: Subscription) -> Self {
        F2BMessage::Unsubscribe(msg)
    }
}

impl TryFrom<F2BMessage> for gloo::net::websocket::Message {
    type Error = bincode::error::EncodeError;

    fn try_from(msg: F2BMessage) -> Result<Self, Self::Error> {
        let bytes = bincode::serde::encode_to_vec(&msg, bincode::config::standard())?;
        Ok(gloo::net::websocket::Message::Bytes(bytes))
    }
}

#[cfg(feature = "backend")]
impl TryFrom<actix_web::web::Bytes> for F2BMessage {
    type Error = bincode::error::DecodeError;

    fn try_from(bytes: actix_web::web::Bytes) -> Result<Self, Self::Error> {
        let (message, number_of_bytes) =
            bincode::serde::decode_from_slice(&bytes, bincode::config::standard())?;
        if number_of_bytes < bytes.len() {
            return Err(bincode::error::DecodeError::UnexpectedEnd {
                additional: bytes.len() - number_of_bytes,
            });
        }
        Ok(message)
    }
}
