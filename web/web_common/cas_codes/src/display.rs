//! Submodule implementing the `Display` trait for the `CAS` number struct.

use std::fmt::Display;

impl Display for crate::CAS {
    /// Formats the CAS number as a string.
    ///
    /// The CAS number is formatted as `XXXX-XX-X`, where:
    /// - `XXXX` is the first part of the CAS number,
    /// - `XX` is the second part of the CAS number,
    /// - `X` is the check digit.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{}", self.first(), self.second(), self.check_digit())
    }
}
