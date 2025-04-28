//! Submodule implementing the `AsRef<[char]>` and `AsRef<[u8]>` trait for
//! country codes.

use crate::country_codes::CountryCode;

impl AsRef<[char]> for CountryCode {
    fn as_ref(&self) -> &[char] {
        <Self as AsRef<[char; 2]>>::as_ref(self).as_ref()
    }
}

impl AsRef<[u8]> for CountryCode {
    fn as_ref(&self) -> &[u8] {
        <Self as AsRef<[u8; 2]>>::as_ref(self).as_ref()
    }
}
