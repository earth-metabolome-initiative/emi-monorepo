//! Error types for CAS number parsing and validation.
//!
//! This module provides comprehensive error handling for CAS number operations,
//! with detailed error messages to help users understand what went wrong.

/// Error types that can occur when parsing or validating CAS numbers.
///
/// ## Examples
///
/// ```rust
/// use cas_codes::{CAS, errors::Error};
///
/// // Invalid format
/// match CAS::try_from("invalid-format") {
///     Err(Error::InvalidString(s)) => {
///         println!("Invalid format: {}", s);
///     }
///     _ => unreachable!(),
/// }
///
/// // Invalid checksum
/// match CAS::try_from("7732-18-6") {
///     Err(Error::InvalidChecksum { cas, expected, actual }) => {
///         println!("Wrong checksum for {}: expected {}, got {}", cas, expected, actual);
///     }
///     _ => unreachable!(),
/// }
/// ```

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Error {
    /// The provided string is not in valid CAS number format.
    ///
    /// Valid formats are:
    /// - `NNNNNNN-NN-N` (using ASCII hyphens)
    /// - `NNNNNNN–NN–N` (using en-dashes U+2013)
    ///
    /// Where N represents digits, with the first part being 2-7 digits,
    /// second part exactly 2 digits, and third part exactly 1 digit.
    InvalidString(String),

    /// The check digit doesn't match the calculated checksum.
    ///
    /// This indicates either a transcription error or corruption in the CAS
    /// number. The checksum is calculated according to the official CAS
    /// algorithm.
    InvalidChecksum {
        /// The provided CAS number string.
        cas: String,
        /// The expected checksum based on the algorithm.
        expected: u8,
        /// The actual check digit found in the string.
        actual: u8,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::InvalidString(s) => write!(f, "Invalid CAS number: {s}"),
            Error::InvalidChecksum { cas, expected, actual } => {
                write!(
                    f,
                    "Invalid checksum for CAS number {cas}: expected {expected}, actual {actual}"
                )
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
