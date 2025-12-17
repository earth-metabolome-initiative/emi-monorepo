//! Defines errors used in the SMILES parser.

/// The errors that could occur during SMILES parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error indicating that an unknown element was encountered.
    Element(elements_rs::errors::Error),
    /// Error indicating that an invalid number was encountered.
    InvalidNumber,
    /// Error indicating that an unexpected character was encountered.
    UnexpectedCharacter {
        /// The unexpected character.
        character: char,
    },
    /// Error indicating a problem in the molecular formula.
    MolecularFormula(molecular_formulas::errors::Error),
}

impl From<molecular_formulas::errors::Error> for Error {
    fn from(err: molecular_formulas::errors::Error) -> Self {
        Error::MolecularFormula(err)
    }
}
