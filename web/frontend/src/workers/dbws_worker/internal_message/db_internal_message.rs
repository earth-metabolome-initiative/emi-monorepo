//! Submodule providing the enumeration of internal messages used
//! for the DB portion of the DB/WebSocket worker.
/// Enumeration of internal messages used for the DB portion of the DB/WebSocket
/// worker.
pub enum DBInternalMessage {
    /// Indicates that the `SAHPool` has been installed successfully.
    SAHPoolInstalled,
}
