//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownElement {
    /// The provided container category string is unknown.
    Unknown([char; 2]),
}

impl std::fmt::Display for UnknownElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownElement::Unknown(code) => {
                write!(f, "Unknown element string: {code:?}")
            }
        }
    }
}

impl std::error::Error for UnknownElement {}
