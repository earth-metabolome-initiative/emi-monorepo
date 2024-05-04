use std::collections::HashSet;

use serde::{Deserialize, Serialize};
pub mod auth;
pub mod documents;
pub mod form_traits;
pub mod oauth;
pub mod ws;

use validator::ValidationError;
use validator::ValidationErrors;

use crate::custom_validators::validation_errors::ValidationErrorToString;

pub const ENDPOINT: &str = "/api";
pub const FULL_ENDPOINT: &str = ENDPOINT;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum ApiError {
    Unauthorized,
    ExpiredAuthorization,
    BadGateway,
    BadRequest(Vec<String>),
    InternalServerError,
    InvalidFileFormat(String),
}

impl ApiError {
    pub fn unauthorized() -> Self {
        Self::Unauthorized
    }

    pub fn internal_server_error() -> Self {
        Self::InternalServerError
    }

    pub fn bad_gateway() -> Self {
        Self::BadGateway
    }

    pub fn invalid_file_format<S>(format: S) -> Self
    where
        S: Into<String>,
    {
        Self::InvalidFileFormat(format.into())
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(e: serde_json::Error) -> Self {
        log::error!("Failed to serialize response: {}", e);
        Self::InternalServerError
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(e: ValidationErrors) -> Self {
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<ValidationError> for ApiError {
    fn from(e: ValidationError) -> Self {
        log::error!("Validation error: {:?}", e);
        Self::BadRequest(e.convert_to_string())
    }
}

impl From<bincode::ErrorKind> for ApiError {
    fn from(e: bincode::ErrorKind) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl From<Box<bincode::ErrorKind>> for ApiError {
    fn from(e: Box<bincode::ErrorKind>) -> Self {
        Self::BadRequest(vec![format!("Serialization failure: {}", e)])
    }
}

impl Into<Vec<String>> for ApiError {
    fn into(self) -> Vec<String> {
        match self {
            ApiError::BadRequest(errors) => errors,
            ApiError::Unauthorized => vec!["Unauthorized".to_string()],
            ApiError::ExpiredAuthorization => vec!["Expired Authorization".to_string()],
            ApiError::BadGateway => vec!["Bad Gateway".to_string()],
            ApiError::InternalServerError => vec!["Internal Server Error".to_string()],
            ApiError::InvalidFileFormat(format) => vec![format!("Invalid file format: {}", format)],
        }
    }
}

impl From<Vec<String>> for ApiError {
    fn from(errors: Vec<String>) -> Self {
        ApiError::BadRequest(errors)
    }
}

impl Into<HashSet<String>> for ApiError {
    fn into(self) -> HashSet<String> {
        let vector: Vec<String> = self.into();
        vector.into_iter().collect()
    }
}

#[cfg(feature = "frontend")]
impl From<gluesql::prelude::Error> for ApiError {
    fn from(e: gluesql::prelude::Error) -> Self {
        log::error!("Glue error: {:?}", e);
        Self::InternalServerError
    }
}

#[cfg(feature = "backend")]
impl From<diesel::result::Error> for ApiError {
    fn from(e: diesel::result::Error) -> Self {
        log::error!("Database error: {:?}", e);
        match e {
            diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) => {
                Self::BadRequest(vec!["You are attempting to insert a duplicated entry".to_string()])
            },
            diesel::result::Error::NotFound => Self::BadRequest(vec!["Not found".to_string()]),
            _ => Self::InternalServerError,
        }
    }
}

#[cfg(feature = "backend")]
impl From<image::ImageError> for ApiError {
    fn from(e: image::ImageError) -> Self {
        log::error!("Image error: {:?}", e);
        Self::InternalServerError
    }
}

#[cfg(feature = "backend")]
impl From<ApiError> for actix_web::HttpResponse {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::Unauthorized => actix_web::HttpResponse::Unauthorized().finish(),
            ApiError::ExpiredAuthorization => actix_web::HttpResponse::Unauthorized().finish(),
            ApiError::BadGateway => actix_web::HttpResponse::BadGateway().finish(),
            ApiError::BadRequest(errors) => actix_web::HttpResponse::BadRequest().json(errors),
            ApiError::InternalServerError => {
                actix_web::HttpResponse::InternalServerError().finish()
            }
            ApiError::InvalidFileFormat(format) => actix_web::HttpResponse::BadRequest()
                .json(format!("Invalid file format: {}", format)),
        }
    }
}
