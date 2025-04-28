//! Submodule implementing the `From<CountryCode>` trait for `String`.

use crate::country_codes::CountryCode;

impl From<CountryCode> for String {
    fn from(code: CountryCode) -> Self {
        <CountryCode as AsRef<str>>::as_ref(&code).to_owned()
    }
}
