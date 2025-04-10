//! Submodule defining the error enumeration that may be happen within the server.

mod from_impls;

#[derive(Debug)]
/// High-level errors that may occur in the Server.
pub(crate) enum BackendError {
    /// An error that occurred while trying to access the redis database.
    RedisError(redis::RedisError),
    /// An error that occurred while trying to connect to the Postgres database.
    PostgresConnectionError(diesel::ConnectionError),
    /// The connection pool timed out while trying to execute a query.
    PostgresPoolTimeoutError,
    /// An error that occurred while trying to execute a query on the Postgres database.
    PostgresQueryError(diesel::result::Error),
    /// An error that occurred due to some error mis-configuration.
    EnvironmentError(std::env::VarError),
    /// The user attempted an unauthorized action.
    Unauthorized,
    /// A third-party service returned an error.
    RequestError(reqwest::Error),
}
