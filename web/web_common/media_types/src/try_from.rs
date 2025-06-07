//! Submodule providing implementations of the `TryFrom` trait for the
//! [`MediaType`] enumeration.

use std::str::FromStr;

impl TryFrom<&str> for crate::MediaType {
    type Error = crate::errors::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        <Self as FromStr>::from_str(value)
    }
}

impl TryFrom<String> for crate::MediaType {
    type Error = crate::errors::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}
