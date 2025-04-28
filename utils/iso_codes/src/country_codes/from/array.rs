//! Submodule implementing the `From<CountryCode>` trait for `[u8; 2]`.

use crate::country_codes::CountryCode;

impl From<CountryCode> for [u8; 2] {
    fn from(code: CountryCode) -> Self {
        <CountryCode as AsRef<[u8; 2]>>::as_ref(&code).to_owned()
    }
}

impl From<CountryCode> for [char; 2] {
    fn from(code: CountryCode) -> Self {
        <CountryCode as AsRef<[char; 2]>>::as_ref(&code).to_owned()
    }
}
