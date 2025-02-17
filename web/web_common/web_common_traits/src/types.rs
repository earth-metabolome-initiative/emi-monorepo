//! Types that are shared between the crates.

#[cfg(feature = "backend")]
/// The connection type used in the backend.
pub type DBConn = diesel_async::AsyncPgConnection;

#[cfg(feature = "backend")]
/// The connection manager type used in the backend.
pub type DBPool = diesel_async::pooled_connection::bb8::Pool<diesel_async::AsyncPgConnection>;
