//! Represents tokens used in parsing SMILES strings.

use elements_rs::{Element, Isotope};
use molecular_formulas::MolecularFormula;

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
/// Represents a token in a molecular formula.
pub(crate) enum Token {
    /// A molecular formula
    MolecularFormula(MolecularFormula),
    /// An open round bracket
    OpenRoundBracket,
    /// A close round bracket
    CloseRoundBracket,
    /// An open square bracket
    OpenSquareBracket,
    /// A close square bracket
    CloseSquareBracket,
    /// A dot
    Dot,
    /// A dash '-' character
    Dash,
    /// An equal '=' character i.e. a double bond
    Equal,
    /// A hash '#' character i.e. a triple bond
    Hashtag,
    /// A dollar '$' character i.e. a quadruple bond
    Dollar,
    /// A colon ':' character
    Colon,
    /// A foward slash '/' character
    ForwardSlash,
    /// A back slash '\' character
    BackSlash,
}

impl From<Element> for Token {
    fn from(element: Element) -> Self {
        Token::MolecularFormula(element.into())
    }
}

impl From<Isotope> for Token {
    fn from(isotope: Isotope) -> Self {
        Token::MolecularFormula(isotope.into())
    }
}
