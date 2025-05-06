//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownContainerCategory {
    /// The provided container category string is unknown.
    UnknownString(String),
}

impl std::fmt::Display for UnknownContainerCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownContainerCategory::UnknownString(code) => {
                write!(f, "Unknown container category string: {}", code)
            }
        }
    }
}

impl std::error::Error for UnknownContainerCategory {}
