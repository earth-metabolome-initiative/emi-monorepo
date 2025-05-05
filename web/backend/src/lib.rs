//! Submodule for the APIs of the web backend.
pub mod api;
pub mod errors;
pub use api::{LNCommand, ListenNotifyHandle, ListenNotifyServer};
pub use errors::BackendError;

pub(crate) type Conn = diesel_async::AsyncPgConnection;
pub type DBPool = diesel_async::pooled_connection::bb8::Pool<Conn>;
