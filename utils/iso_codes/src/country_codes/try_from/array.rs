//! Submodule providing implementations of the `TryFrom<[u8; 2]>` and
//! `TryFrom<[char; 2]>` traits for `CountryCode`.

use crate::CountryCode;

impl TryFrom<[u8; 2]> for CountryCode {
    type Error = crate::errors::UnknownCountryCode;

    fn try_from(code: [u8; 2]) -> Result<Self, Self::Error> {
        code.as_ref().try_into()
    }
}

impl TryFrom<[char; 2]> for CountryCode {
    type Error = crate::errors::UnknownCountryCode;

    fn try_from(code: [char; 2]) -> Result<Self, Self::Error> {
        code.as_ref().try_into()
    }
}
