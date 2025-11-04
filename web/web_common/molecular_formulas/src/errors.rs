//! Submodule providing the enumeration of errors which may occur while parsing
//! a molecular formula.

use std::num::TryFromIntError;

use crate::token::{Token, greek_letters::GreekLetter};

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Enumeration of errors which may occur while parsing a molecular formula.
pub enum Error {
    /// Error indicating that an unknown element was encountered.
    Element(elements_rs::errors::Error),
    /// Error indicating that a character in the formula is invalid.
    InvalidCharacter(char),
    /// Invalid repeated token in the formula.
    InvalidRepeatedToken(Token),
    /// Error indicating that a greek letter in the formula is at an
    /// invalid position.
    InvalidGreekLetterPosition(GreekLetter),
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
    /// Error raised when an uncountable term is being counted.
    CountingUncountable,
    /// When the leading token is not a number or an element.
    InvalidLeadingToken(Token),
    /// When the parser is not completely consumed.
    UnconsumedParser,
    /// When an ion has a charge of 0.
    ZeroCharge,
    /// When a count has a value of 0.
    ZeroCount,
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
    /// When a provided string is not a valid greek letter.
    InvalidGreekLetter(String),
}

impl From<elements_rs::errors::Error> for Error {
    fn from(err: elements_rs::errors::Error) -> Self {
        Error::Element(err)
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Element(e) => write!(f, "Element error: {e}"),
            Error::InvalidCharacter(c) => write!(f, "Invalid character: {c}"),
            Error::InvalidRepeatedToken(token) => {
                write!(f, "Invalid repeated token: {token:?}")
            }
            Error::InvalidGreekLetterPosition(c) => {
                write!(f, "Invalid greek letter position: {c}")
            }
            Error::InvalidNumber => write!(f, "Invalid number"),
            Error::EmptyFormula => write!(f, "Empty formula"),
            Error::InvalidFormula => write!(f, "Invalid formula"),
            Error::ClosingToken { expected, found } => {
                write!(f, "Expected closing token: {expected:?}, found: {found:?}")
            }
            Error::CountingUncountable => write!(f, "Counting uncountable term"),
            Error::InvalidLeadingToken(token) => {
                write!(f, "Invalid leading token: {token:?}")
            }
            Error::UnconsumedParser => write!(f, "Unconsumed parser"),
            Error::ZeroCharge => write!(f, "Ion has a charge of 0"),
            Error::ZeroCount => write!(f, "Count has a value of 0"),
            Error::InvalidChargePosition => write!(f, "Charge is not at the end of the formula"),
            Error::InvalidSuperscriptPosition => {
                write!(f, "Superscript is at an invalid position")
            }
            Error::InvalidOperationForResidual => {
                write!(f, "Operation is not defined for residuals")
            }
            Error::InvalidOperationForMixture => {
                write!(f, "Operation is not defined for mixtures")
            }
            Error::InvalidOperationForNonDiatomic => {
                write!(f, "Operation is only defined for diatomic formulas")
            }
            Error::InvalidOxidationState(state) => {
                write!(f, "Oxidation state is invalid: {state}")
            }
            Error::InvalidGreekLetter(greek) => {
                write!(f, "Provided string is not a valid greek letter: {greek}")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::InvalidNumber
    }
}
