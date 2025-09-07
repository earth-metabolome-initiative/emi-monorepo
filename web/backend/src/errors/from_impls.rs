//! Submodule implementing the `From` trait for various backend error types.

use actix_web::HttpResponse;
use backend_request_errors::BackendRequestError;
use core_structures::tables::insertables::{
    EmailProviderAttribute, TemporaryUserAttribute, UserEmailAttribute,
};
use generic_backend_request_errors::GenericBackendRequestError;
use tokio::sync::{mpsc::error::SendError, oneshot::error::RecvError};
use web_common_traits::database::InsertError;

use super::BackendError;
use crate::{LNCommand, api::oauth::jwt_cookies::eliminate_cookies};

impl From<diesel::result::Error> for BackendError {
    fn from(error: diesel::result::Error) -> Self {
        BackendError::PostgresQuery(error)
    }
}

impl From<diesel::r2d2::PoolError> for BackendError {
    fn from(error: diesel::r2d2::PoolError) -> Self {
        BackendError::PostgresPool(error)
    }
}

impl From<diesel::ConnectionError> for BackendError {
    fn from(connection_error: diesel::ConnectionError) -> Self {
        BackendError::PostgresConnection(connection_error)
    }
}

impl From<std::env::VarError> for BackendError {
    fn from(error: std::env::VarError) -> Self {
        BackendError::Environment(error)
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(error: reqwest::Error) -> Self {
        BackendError::Request(error)
    }
}

impl From<redis::RedisError> for BackendError {
    fn from(error: redis::RedisError) -> Self {
        BackendError::Redis(error)
    }
}

impl From<std::num::ParseIntError> for BackendError {
    fn from(error: std::num::ParseIntError) -> Self {
        BackendError::ParseInt(error)
    }
}

impl From<std::string::FromUtf8Error> for BackendError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        BackendError::FromUtf8(error)
    }
}

impl From<base64::DecodeError> for BackendError {
    fn from(error: base64::DecodeError) -> Self {
        BackendError::Base64Decode(error)
    }
}

impl From<jsonwebtoken::errors::Error> for BackendError {
    fn from(error: jsonwebtoken::errors::Error) -> Self {
        BackendError::JWT(error)
    }
}

impl From<BackendError> for BackendRequestError {
    fn from(error: BackendError) -> Self {
        log::debug!("BackendError: {error:?}");
        match error {
            BackendError::PostgresConnection(_)
            | BackendError::Environment(_)
            | BackendError::PostgresPool(_)
            | BackendError::PostgresQuery(_)
            | BackendError::Redis(_)
            | BackendError::ParseInt(_)
            | BackendError::FromUtf8(_)
            | BackendError::Base64Decode(_)
            | BackendError::JWT(_)
            | BackendError::UnknownLoginProvider(_)
            | BackendError::ListenNotify
            | BackendError::EmailProviderInsert(_)
            | BackendError::TemporaryUserInsert(_)
            | BackendError::Request(_) => {
                BackendRequestError::Generic(GenericBackendRequestError::InternalServerError)
            }
            BackendError::Unauthorized | BackendError::LoginCollision => {
                BackendRequestError::Generic(GenericBackendRequestError::Unauthorized)
            }
            BackendError::UserEmailInsert(error) => BackendRequestError::UserEmailInsert(error),
        }
    }
}

impl From<BackendError> for HttpResponse {
    fn from(error: BackendError) -> Self {
        let backend_request_error = BackendRequestError::from(error);
        match backend_request_error {
            BackendRequestError::Generic(GenericBackendRequestError::InternalServerError)
            | BackendRequestError::UserEmailInsert(_) => {
                eliminate_cookies(HttpResponse::InternalServerError())
            }
            BackendRequestError::Generic(GenericBackendRequestError::NotFound) => {
                HttpResponse::NotFound().finish()
            }
            BackendRequestError::Generic(GenericBackendRequestError::Unauthorized) => {
                eliminate_cookies(HttpResponse::Unauthorized())
            }
        }
    }
}

impl From<BackendError> for actix_web::error::Error {
    fn from(error: BackendError) -> Self {
        let backend_request_error = BackendRequestError::from(error);
        match backend_request_error {
            BackendRequestError::Generic(GenericBackendRequestError::InternalServerError)
            | BackendRequestError::UserEmailInsert(_) => {
                actix_web::error::ErrorInternalServerError(
                    serde_json::json!({"status": "fail", "message": "Internal server error"}),
                )
            }
            BackendRequestError::Generic(GenericBackendRequestError::NotFound) => {
                actix_web::error::ErrorNotFound(
                    serde_json::json!({"status": "fail", "message": "Not found"}),
                )
            }
            BackendRequestError::Generic(GenericBackendRequestError::Unauthorized) => {
                actix_web::error::ErrorUnauthorized(
                    serde_json::json!({"status": "fail", "message": "Unauthorized"}),
                )
            }
        }
    }
}

impl From<SendError<LNCommand>> for BackendError {
    fn from(_error: SendError<LNCommand>) -> Self {
        Self::ListenNotify
    }
}

impl From<RecvError> for BackendError {
    fn from(_error: RecvError) -> Self {
        Self::ListenNotify
    }
}

impl From<InsertError<UserEmailAttribute>> for BackendError {
    fn from(error: InsertError<UserEmailAttribute>) -> Self {
        BackendError::UserEmailInsert(error)
    }
}

impl From<InsertError<EmailProviderAttribute>> for BackendError {
    fn from(error: InsertError<EmailProviderAttribute>) -> Self {
        BackendError::EmailProviderInsert(error)
    }
}

impl From<InsertError<TemporaryUserAttribute>> for BackendError {
    fn from(error: InsertError<TemporaryUserAttribute>) -> Self {
        BackendError::TemporaryUserInsert(error)
    }
}
