//! Submodule providing the enumeration of errors which may occur while parsing
//! a molecular formula.

use crate::token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
/// Enumeration of errors which may occur while parsing a molecular formula.
pub enum Error {
    /// Error indicating that an unknown element was encountered.
    Element(elements::errors::Error),
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
    /// When an ion has a charge of 0.
    ZeroCharge,
    /// When a charge is not at the end of the formula.
    InvalidChargePosition,
    /// When a superscript is at an invalid position.
    InvalidSuperscriptPosition,
    /// When an operation is not defined for residuals.
    InvalidOperationForResidual,
    /// When an operation is not defined for a mixture.
    InvalidOperationForMixture,
    /// When an operation is only defined for diatomic formulas.
    InvalidOperationForNonDiatomic,
    /// When an oxidation state is invalid.
    InvalidOxidationState(i16),
}

impl From<elements::errors::Error> for Error {
    fn from(err: elements::errors::Error) -> Self {
        Error::Element(err)
    }
}
