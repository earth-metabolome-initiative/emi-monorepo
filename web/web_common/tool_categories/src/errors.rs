//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownToolCategory {
    /// The provided tool category string is unknown.
    UnknownString(String),
}

impl std::fmt::Display for UnknownToolCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownToolCategory::UnknownString(code) => {
                write!(f, "Unknown tool category string: {code}")
            }
        }
    }
}

impl std::error::Error for UnknownToolCategory {}
