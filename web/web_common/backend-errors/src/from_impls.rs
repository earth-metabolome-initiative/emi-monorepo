//! Submodule implementing the `From` trait for various error types.
use diesel_async::pooled_connection::bb8::RunError;

use crate::Error;

impl From<diesel::result::Error> for Error {
    fn from(_: diesel::result::Error) -> Self {
        Error::DieselError
    }
}

impl From<diesel::ConnectionError> for Error {
    fn from(_: diesel::ConnectionError) -> Self {
        Error::DieselError
    }
}

impl From<RunError> for Error {
    fn from(_: RunError) -> Self {
        Error::DieselError
    }
}

impl From<std::env::VarError> for Error {
    fn from(_: std::env::VarError) -> Self {
        Error::EnvironmentError
    }
}

impl From<reqwest::Error> for Error {
    fn from(_: reqwest::Error) -> Self {
        Error::ThirdPartyError
    }
}

impl From<redis::RedisError> for Error {
    fn from(_: redis::RedisError) -> Self {
        Error::RedisError
    }
}
