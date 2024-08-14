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

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum ApiError {
    Unauthorized,
    BadGateway,
    NoResults,
    BadRequest(Vec<String>),
    Empty(String),
    InternalServerError,
    InvalidFileFormat(String),
    JPEGError(JPEGError),
    DeviceError(DeviceError),
    Geolocation(GeolocationError),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum JPEGError {
    InvalidImage,
    ImageTooSmall,
    ImageHasTransparency,
    ImageTooDark,
    ImageTooLight,
    ImageIsBlurry,
    ImageHasFewColors,
    UnableToEncode,
}

impl ToString for JPEGError {
    fn to_string(&self) -> String {
        match self {
            JPEGError::InvalidImage => "Invalid JPEG file.".to_string(),
            JPEGError::ImageTooSmall => "Image is too small. Minimum size is 64x64.".to_string(),
            JPEGError::ImageHasTransparency => "Image has transparency.".to_string(),
            JPEGError::ImageTooDark => "The provided image is too dark.".to_string(),
            JPEGError::ImageTooLight => "The provided image is too light.".to_string(),
            JPEGError::ImageIsBlurry => "The provided image is blurry.".to_string(),
            JPEGError::ImageHasFewColors => concat!(
                "The image contains a very limited number of colors. ",
                "Therefore, it is unlikely to be a photograph."
            )
            .to_string(),
            JPEGError::UnableToEncode => "Unable to encode image.".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum DeviceError {
    NoCameras,
    DeviceStoppedResponding,
}

impl ToString for DeviceError {
    fn to_string(&self) -> String {
        match self {
            DeviceError::NoCameras => "No cameras found.".to_string(),
            DeviceError::DeviceStoppedResponding => "Device stopped responding.".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Hash, PartialOrd, Eq, Ord)]
pub enum GeolocationError {
    NotSupported,
    PermissionDenied,
    PositionUnavailable,
    Timeout,
    Unknown(String),
}

impl ToString for GeolocationError {
    fn to_string(&self) -> String {
        match self {
            GeolocationError::NotSupported => "Geolocation is not supported.".to_string(),
            GeolocationError::PermissionDenied => "Permission to geolocate was denied - You may need to authorize this in your device privacy settings.".to_string(),
            GeolocationError::PositionUnavailable => "Position unavailable.".to_string(),
            GeolocationError::Timeout => "Geolocation request timed out.".to_string(),
            GeolocationError::Unknown(e) => format!("Geolocation error: {}", e),
        }
    }
}

impl ApiError {
    pub fn font_awesome_icon(&self) -> &'static str {
        match self {
            Self::Unauthorized => "user-secret",
            Self::BadGateway => "dungeon",
            Self::BadRequest(_) => "circle-exclamation",
            Self::InternalServerError => "bomb",
            Self::NoResults => "search",
            Self::InvalidFileFormat(_) => "file-circle-exclamation",
            Self::Empty(_) => "puzzle-piece",
            Self::JPEGError(e) => match e {
                JPEGError::InvalidImage => "file-image",
                JPEGError::ImageTooSmall => "expand-arrows-alt",
                JPEGError::ImageHasTransparency => "file-image",
                JPEGError::ImageTooDark => "moon",
                JPEGError::ImageTooLight => "sun",
                JPEGError::ImageIsBlurry => "eye-slash",
                JPEGError::ImageHasFewColors => "palette",
                JPEGError::UnableToEncode => "file-image",
            },
            Self::DeviceError(e) => match e {
                DeviceError::NoCameras => "camera",
                DeviceError::DeviceStoppedResponding => "heart-crack",
            },
            Self::Geolocation(e) => match e {
                GeolocationError::NotSupported => "person-circle-question",
                GeolocationError::PermissionDenied => "lock",
                GeolocationError::PositionUnavailable => "person-circle-question",
                GeolocationError::Timeout => "stopwatch",
                GeolocationError::Unknown(_) => "map-location-dot",
            },
        }
    }

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

impl From<String> for ApiError {
    fn from(e: String) -> Self {
        Self::BadRequest(vec![e])
    }
}

impl From<&str> for ApiError {
    fn from(e: &str) -> Self {
        Self::BadRequest(vec![e.to_string()])
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
        log::error!("ApiError: {:?}", self);
        match self {
            ApiError::BadRequest(errors) => errors,
            ApiError::NoResults => vec!["No results".to_string()],
            ApiError::Unauthorized => vec!["Unauthorized".to_string()],
            ApiError::BadGateway => vec!["Bad Gateway".to_string()],
            ApiError::Empty(e) => vec![format!("Field `{}` cannot be empty", e)],
            ApiError::InternalServerError => vec!["Internal Server Error".to_string()],
            ApiError::InvalidFileFormat(format) => vec![format!("Invalid file format: {}", format)],
            ApiError::JPEGError(e) => vec![e.to_string()],
            ApiError::DeviceError(e) => vec![e.to_string()],
            ApiError::Geolocation(e) => vec![e.to_string()],
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

impl From<reqwest::Error> for ApiError {
    fn from(e: reqwest::Error) -> Self {
        log::error!("Reqwest error: {:?}", e);
        Self::BadGateway
    }
}

impl From<JPEGError> for ApiError {
    fn from(e: JPEGError) -> Self {
        Self::JPEGError(e)
    }
}

impl From<DeviceError> for ApiError {
    fn from(e: DeviceError) -> Self {
        Self::DeviceError(e)
    }
}

impl From<GeolocationError> for ApiError {
    fn from(e: GeolocationError) -> Self {
        Self::Geolocation(e)
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
        match e {
            diesel::result::Error::DatabaseError(kind, information) => {
                log::error!("Database error {:?}: message: {:?}, details: {:?}, hint: {:?}, table_name: {:?}, column_name: {:?}, constraint_name: {:?}, statement_position: {:?}", kind, information.message(), information.details(), information.hint(), information.table_name(), information.column_name(), information.constraint_name(), information.statement_position());

                match kind {
                    diesel::result::DatabaseErrorKind::UniqueViolation => Self::BadRequest(vec![
                        "You are attempting to create a duplicate entry".to_string(),
                    ]),
                    diesel::result::DatabaseErrorKind::ForeignKeyViolation => {
                        Self::BadRequest(vec![
                            "You are attempting to create a reference to a non-existent entry"
                                .to_string(),
                        ])
                    }
                    _ => match information.message() {
                        "unauthorized_update" => Self::Unauthorized,
                        _ => Self::InternalServerError,
                    },
                }
            }
            diesel::result::Error::NotFound => {
                Self::BadRequest(vec!["Diesel error: Not found".to_string()])
            }
            e => {
                log::error!("Database error: {:?}", e);
                Self::InternalServerError
            }
        }
    }
}

#[cfg(feature = "backend")]
impl From<diesel::r2d2::Error> for ApiError {
    fn from(e: diesel::r2d2::Error) -> Self {
        log::error!("Database connection error: {:?}", e);
        Self::InternalServerError
    }
}

#[cfg(feature = "backend")]
impl From<r2d2::Error> for ApiError {
    fn from(e: r2d2::Error) -> Self {
        log::error!("Database connection error: {:?}", e);
        Self::InternalServerError
    }
}

impl From<image::ImageError> for ApiError {
    fn from(e: image::ImageError) -> Self {
        log::error!("Image error: {:?}", e);
        Self::InternalServerError
    }
}

#[cfg(feature = "frontend")]
impl From<wasm_bindgen::JsValue> for ApiError {
    fn from(e: wasm_bindgen::JsValue) -> Self {
        log::error!("JsValue error: {:?}", e);
        Self::InternalServerError
    }
}

#[cfg(feature = "frontend")]
impl From<web_sys::PositionError> for GeolocationError {
    fn from(e: web_sys::PositionError) -> Self {
        match e.code() {
            web_sys::PositionError::PERMISSION_DENIED => GeolocationError::PermissionDenied,
            web_sys::PositionError::POSITION_UNAVAILABLE => GeolocationError::PositionUnavailable,
            web_sys::PositionError::TIMEOUT => GeolocationError::Timeout,
            unknown => GeolocationError::Unknown(format!("Position error code: {:?}", unknown)),
        }
    }
}

#[cfg(feature = "backend")]
impl From<ApiError> for actix_web::HttpResponse {
    fn from(e: ApiError) -> Self {
        match e {
            ApiError::Unauthorized => actix_web::HttpResponse::Unauthorized().finish(),
            ApiError::BadGateway => actix_web::HttpResponse::BadGateway().finish(),
            ApiError::BadRequest(errors) => actix_web::HttpResponse::BadRequest().json(errors),
            ApiError::NoResults => actix_web::HttpResponse::NotFound().finish(),
            ApiError::Empty(e) => {
                actix_web::HttpResponse::BadRequest().json(format!("Field `{}` cannot be empty", e))
            }
            ApiError::InternalServerError => {
                actix_web::HttpResponse::InternalServerError().finish()
            }
            ApiError::InvalidFileFormat(format) => actix_web::HttpResponse::BadRequest()
                .json(format!("Invalid file format: {}", format)),
            ApiError::JPEGError(e) => actix_web::HttpResponse::BadRequest().json(e.to_string()),
            ApiError::DeviceError(e) => actix_web::HttpResponse::BadRequest().json(e.to_string()),
            ApiError::Geolocation(e) => actix_web::HttpResponse::BadRequest().json(e.to_string()),
        }
    }
}
