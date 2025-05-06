//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownNameplateCategory {
    /// The provided nameplate category string is unknown.
    UnknownString(String),
}

impl std::fmt::Display for UnknownNameplateCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownNameplateCategory::UnknownString(code) => {
                write!(f, "Unknown nameplate category string: {code}")
            }
        }
    }
}

impl std::error::Error for UnknownNameplateCategory {}
