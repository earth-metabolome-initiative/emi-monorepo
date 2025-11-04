//! Error types for element and isotope parsing.

use crate::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Errors that can occur when parsing elements or isotopes.
pub enum Error {
    /// The provided element is unknown.
    Element([char; 2]),
    /// The provided character isotope is unknown.
    CharacterIsotope(char),
    /// The provided combination of Element and atomic mass is unknown.
    Isotope(Element, u16),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Element(code) => {
                write!(f, "Unknown element string: {code:?}")
            }
            Error::CharacterIsotope(c) => {
                write!(f, "Unknown character isotope: {c}")
            }
            Error::Isotope(element, mass) => {
                write!(f, "Unknown isotope: {element} with atomic mass {mass}")
            }
        }
    }
}

impl std::error::Error for Error {}
