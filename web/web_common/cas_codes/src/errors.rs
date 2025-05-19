//! Submodule providing the error enumeration for the errors which may occur
//! when parsing a CAS number.

#[derive(Debug, Clone, PartialEq, Eq)]
/// Error enumeration for CAS number parsing errors.
pub enum Error {
    /// When the provided string is not a valid CAS number.
    InvalidString(String),
    /// When the check sum does not match the expected value.
    InvalidChecksum {
        /// The provided CAS number.
        cas: String,
        /// The expected checksum.
        expected: u8,
        /// The actual checksum.
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
