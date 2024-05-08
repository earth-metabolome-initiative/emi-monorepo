use chrono::NaiveDateTime;
use validator::{ValidationError, ValidationErrors, ValidationErrorsKind};

use crate::api::ApiError;

pub trait ValidationErrorToString {
    fn convert_to_string(&self) -> Vec<String>;
}

impl ValidationErrorToString for ValidationError {
    fn convert_to_string(&self) -> Vec<String> {
        vec![self
            .message
            .clone()
            .map(|message| format!("{}", message))
            .unwrap_or_else(|| self.code.to_string())]
    }
}

impl ValidationErrorToString for ValidationErrors {
    fn convert_to_string(&self) -> Vec<String> {
        self.errors()
            .iter()
            .flat_map(|(_, errors)| match errors {
                ValidationErrorsKind::Field(errors) => errors
                    .iter()
                    .flat_map(|error| error.convert_to_string())
                    .collect::<Vec<String>>(),
                ValidationErrorsKind::List(errors) => errors
                    .iter()
                    .flat_map(|(_, error)| error.convert_to_string())
                    .collect::<Vec<String>>(),
                ValidationErrorsKind::Struct(errors) => errors.convert_to_string(),
            })
            .collect::<Vec<String>>()
    }
}

pub trait TryFromString: Sized {
    fn try_from_string(value: String) -> Result<Self, ApiError>;
}

impl TryFromString for String {
    fn try_from_string(value: String) -> Result<Self, ApiError> {
        Ok(value)
    }
}

impl TryFromString for i32 {
    fn try_from_string(value: String) -> Result<Self, ApiError> {
        value.parse::<i32>().map_err(|_| {
            ApiError::BadRequest(vec![
                "The provided value is not a valid signed 32-bit integer.".to_string(),
            ])
        })
    }
}

impl TryFromString for i64 {
    fn try_from_string(value: String) -> Result<Self, ApiError> {
        value.parse::<i64>().map_err(|_| {
            ApiError::BadRequest(vec![
                "The provided value is not a valid signed 64-bit integer.".to_string(),
            ])
        })
    }
}

impl TryFromString for f64 {
    fn try_from_string(value: String) -> Result<Self, ApiError> {
        value.parse::<f64>().map_err(|_| {
            ApiError::BadRequest(vec![
                "The provided value is not a valid floating point number.".to_string(),
            ])
        })
    }
}