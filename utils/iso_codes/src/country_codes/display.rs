//! Submodule implementing the `Display` trait for country codes.

use std::fmt::Display;

impl Display for super::CountryCode {
    /// Formats the country code as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use iso_codes::CountryCode;
    ///
    /// let code = CountryCode::from("US");
    /// assert_eq!(code.to_string(), "US");
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <Self as AsRef<str>>::as_ref(self))
    }
}
