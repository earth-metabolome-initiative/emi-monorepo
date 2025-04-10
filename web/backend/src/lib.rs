//! Submodule for the APIs of the web backend.
pub mod api;
pub(crate) mod errors;

pub(crate) type Conn = diesel_async::AsyncPgConnection;
pub type DBPool = diesel_async::pooled_connection::bb8::Pool<Conn>;
