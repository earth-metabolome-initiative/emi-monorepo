//! Submodule providing implementations of the `TryFrom<str>` trait for
//! `CountryCode`.

use crate::CountryCode;

impl TryFrom<&str> for CountryCode {
    type Error = crate::errors::UnknownCountryCode;

    fn try_from(code: &str) -> Result<Self, Self::Error> {
        code.as_bytes().try_into()
    }
}
