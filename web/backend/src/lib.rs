//! Submodule for the APIs of the web backend.
pub mod api;
pub mod errors;
pub(crate) use api::LNCommand;
pub use api::{ListenNotifyHandle, ListenNotifyServer};
pub use errors::BackendError;

pub(crate) type Conn = diesel::PgConnection;

/// Diesel connection pool type
pub type DBPool = diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<Conn>>;
