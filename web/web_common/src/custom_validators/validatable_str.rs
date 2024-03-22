//! Submodule providing structs that transparently wrap strings and str references and implement
//! the validator::Validate trait.

use serde::{Serialize, Deserialize};
use validator::Validate;

use super::validation_errors::TryFromString;


#[derive(Debug, Clone, Serialize, PartialEq, Eq, Default, Hash, Deserialize)]
pub struct ValidatableString {
    value: String,
}

impl TryFromString for ValidatableString {
    fn try_from_string(value: String) -> Result<Self, Vec<String>> {
        Ok(Self { value })
    }
}

impl From<String> for ValidatableString {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for ValidatableString {
    fn from(value: &str) -> Self {
        Self { value: value.to_string() }
    }
}

impl Validate for ValidatableString {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

impl AsRef<str> for ValidatableString {
    fn as_ref(&self) -> &str {
        &self.value
    }
}

impl AsRef<String> for ValidatableString {
    fn as_ref(&self) -> &String {
        &self.value
    }
}