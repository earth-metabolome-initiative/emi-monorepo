//! Error codes enumeration which may be returned by the library.

use crate::RootMediaType;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum Error {
    /// The media type is unknown.
    UnknownMediaType(String),
    /// The root media type is unknown.
    UnknownRootMediaType(String),
    /// The sub media type is unknown.
    UnknownSubMediaType(RootMediaType, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnknownMediaType(code) => {
                write!(f, "Unknown media type: `{code}`")
            }
            Error::UnknownRootMediaType(code) => {
                write!(f, "Unknown root media type: `{code}`")
            }
            Error::UnknownSubMediaType(root, code) => {
                write!(f, "Unknown sub media type: `{code}` for root `{root}`")
            }
        }
    }
}

impl std::error::Error for Error {}
