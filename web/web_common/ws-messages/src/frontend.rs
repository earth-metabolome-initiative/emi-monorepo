//! Submodule providing the enumeration of the websocket messages from the
//! frontend to the backend.

/// Websocket messages from the frontend to the backend.
pub enum F2BMessage {
    /// Health check message.
    Ping,
}
