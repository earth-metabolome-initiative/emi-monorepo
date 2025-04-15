//! Submodule providing the enumeration of the websocket messages from the
//! backend to the frontend.

/// Websocket messages from the backend to the frontend.
pub enum B2FMessage {
    /// Health check message reply.
    Pong,
}
