//! Submodule providing the enumeration of the websocket messages from the
//! frontend to the backend.

use core_structures::tables::read::F2BReadAll;
use web_common_traits::crud::ReadAll;

/// Websocket messages from the frontend to the backend.
pub enum F2BMessage {
    /// Health check message.
    Ping,
    /// A `ReadAll` message from the frontend.
    F2BReadAll(F2BReadAll),
}

impl From<F2BReadAll> for F2BMessage {
    fn from(msg: F2BReadAll) -> Self {
        F2BMessage::F2BReadAll(msg)
    }
}

impl<T> From<ReadAll<T>> for F2BMessage
where
    ReadAll<T>: Into<F2BReadAll>,
{
    fn from(msg: ReadAll<T>) -> Self {
        F2BMessage::F2BReadAll(msg.into())
    }
}
