//! Submodule providing the enumeration of errors which may occur while parsing
//! a molecular formula.

use elements::errors::UnknownElement;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
/// Enumeration of errors which may occur while parsing a molecular formula.
pub enum Error {
    /// Error indicating that an unknown element was encountered.
    Element(UnknownElement),
    /// Error indicating that a character in the formula is invalid.
    InvalidCharacter(char),
    /// Error indicating that a number in the formula is invalid.
    InvalidNumber,
    /// Error indicating that a number in the formula is invalid.
    EmptyFormula,
    /// Error indicating that a formula is invalid.
    InvalidFormula,
    /// Error indicating that the expected closing token was not found.
    ClosingToken {
        /// The expected closing token.
        expected: Option<Token>,
        /// The found closing token.
        found: Option<Token>,
    },
    /// When the leading token is not a number or an element.
    InvalidLeadingToken(Token),
    /// When the parser is not completely consumed.
    UnconsumedParser,
}

impl From<UnknownElement> for Error {
    fn from(err: UnknownElement) -> Self {
        Error::Element(err)
    }
}
