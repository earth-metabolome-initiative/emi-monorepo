//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownInstrumentCategory {
    /// The provided instrument category string is unknown.
    UnknownString(String),
}

impl std::fmt::Display for UnknownInstrumentCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownInstrumentCategory::UnknownString(code) => {
                write!(f, "Unknown instrument category string: {}", code)
            }
        }
    }
}

impl std::error::Error for UnknownInstrumentCategory {}
