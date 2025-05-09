//! Error codes enumeration which may be returned by the library.

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Error codes enumeration which may be returned by the library.
pub enum Error {
    /// The provided container category string is unknown.
    Element([char; 2]),
    /// The provided combination of Element and atomic mass is unknown.
    Isotope(Element, u16)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Element(code) => {
                write!(f, "Unknown element string: {code:?}")
            }
            Error::Isotope(element, mass) => {
                write!(f, "Unknown isotope: {element} with atomic mass {mass}")
            }
        }
    }
}

impl std::error::Error for Error {}
