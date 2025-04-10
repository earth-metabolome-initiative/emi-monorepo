//! Submodule implementing the `From` trait for various backend error types.

use diesel_async::pooled_connection::{PoolError, bb8::RunError};

use super::BackendError;

impl From<diesel::result::Error> for BackendError {
    fn from(error: diesel::result::Error) -> Self {
        BackendError::PostgresQueryError(error)
    }
}

impl From<diesel::ConnectionError> for BackendError {
    fn from(connection_error: diesel::ConnectionError) -> Self {
        BackendError::PostgresConnectionError(connection_error)
    }
}

impl From<RunError> for BackendError {
    fn from(bb8_error: RunError) -> Self {
        match bb8_error {
            RunError::TimedOut => BackendError::PostgresPoolTimeoutError,
            RunError::User(PoolError::ConnectionError(connection_error)) => {
                BackendError::PostgresConnectionError(connection_error)
            }
            RunError::User(PoolError::QueryError(query_error)) => {
                BackendError::PostgresQueryError(query_error)
            }
        }
    }
}

impl From<std::env::VarError> for BackendError {
    fn from(error: std::env::VarError) -> Self {
        BackendError::EnvironmentError(error)
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(error: reqwest::Error) -> Self {
        BackendError::RequestError(error)
    }
}

impl From<redis::RedisError> for BackendError {
    fn from(error: redis::RedisError) -> Self {
        BackendError::RedisError(error)
    }
}
