//! Submodule defining the error enumeration that may be happen within the
//! server.

use core_structures::tables::insertables::{
    InsertableEmailProviderAttributes, InsertableTemporaryUserAttributes,
    InsertableUserEmailAttributes,
};
use web_common_traits::database::InsertError;

mod from_impls;

#[derive(Debug)]
#[allow(dead_code)]
/// High-level errors that may occur in the Server.
pub enum BackendError {
    /// An error that occurred while trying to access the redis database.
    Redis(redis::RedisError),
    /// An error that occurred while trying to connect to the Postgres database.
    PostgresConnection(diesel::ConnectionError),
    /// The connection pool timed out while trying to execute a query.
    PostgresPool(diesel::r2d2::PoolError),
    /// An error that occurred while trying to execute a query on the Postgres
    /// database.
    PostgresQuery(diesel::result::Error),
    /// An error that occurred due to some error mis-configuration.
    Environment(std::env::VarError),
    /// The user attempted an unauthorized action.
    Unauthorized,
    /// A third-party service returned an error.
    Request(reqwest::Error),
    /// An error happened while parsing a value to an integer.
    ParseInt(std::num::ParseIntError),
    /// An error happened while trying to decode a base64 value.
    Base64Decode(base64::DecodeError),
    /// An error happened while trying to decode a JSON value.
    FromUtf8(std::string::FromUtf8Error),
    /// An error occurred while encoding or decoding a `JSONWebToken`.
    JWT(jsonwebtoken::errors::Error),
    /// When inserting a new user email failed.
    UserEmailInsert(InsertError<InsertableUserEmailAttributes>),
    /// When inserting a new email provider failed.
    EmailProviderInsert(InsertError<InsertableEmailProviderAttributes>),
    /// When inserting a temporary user failed.
    TemporaryUserInsert(InsertError<InsertableTemporaryUserAttributes>),
    /// When a login provider is not found in the database.
    UnknownLoginProvider(String),
    /// When there is a failure in the `ListenNotify` server.
    ListenNotify,
    /// When there is a collision between two users.
    LoginCollision,
}
