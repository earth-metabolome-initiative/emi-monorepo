#![doc = include_str!("../README.md")]

use core_structures::tables::insertables::InsertableUserEmailAttributes;
use generic_backend_request_errors::GenericBackendRequestError;
use web_common_traits::database::InsertError;

#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
/// High-level errors that may occur in the Server.
pub enum BackendRequestError {
    /// An error that occurred on the server.
    Generic(GenericBackendRequestError),
    /// When inserting a new user email failed.
    UserEmailInsert(InsertError<InsertableUserEmailAttributes>),
}

impl core::fmt::Display for BackendRequestError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::Generic(error) => write!(f, "{error}"),
            Self::UserEmailInsert(error) => write!(f, "{error}"),
        }
    }
}
