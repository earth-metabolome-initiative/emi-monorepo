//! Error codes enumeration which may be returned by the library.

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum UnknownCountryCode {
    /// The provided country code array is unknown.
    UnknownArray(Vec<u8>),
    /// The provided country code string is unknown.
    UnknownString(String),
    /// The provided country code char array is unknown.
    UnknownCharArray(Vec<char>),
}

impl std::fmt::Display for UnknownCountryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnknownCountryCode::UnknownArray(code) => {
                write!(f, "Unknown country code array: {:?}", code)
            }
            UnknownCountryCode::UnknownString(code) => {
                write!(f, "Unknown country code string: {}", code)
            }
            UnknownCountryCode::UnknownCharArray(code) => {
                write!(f, "Unknown country code char array: {:?}", code)
            }
        }
    }
}

impl std::error::Error for UnknownCountryCode {}
