//! Submodule providing a `Token` enumeration with the entries which may appear
//! in a molecular formula.

use elements_rs::{Element, Isotope};
use greek_letters::GreekLetter;
pub mod greek_letters;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a token in a molecular formula.
pub enum Token {
    /// An element
    Element(Element),
    /// An isotope
    Isotope(Isotope),
    /// A charge
    Charge(i16),
    /// A subscript number, which may be a count
    Count(u16),
    /// A residual group
    Residual,
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
    /// A greek letter
    Greek(GreekLetter),
}

impl Token {
    /// Returns the associated closing token for the given opening token.
    ///
    /// # Panics
    ///
    /// * If the token is not an opening bracket.
    pub(crate) fn closing_token(self) -> Token {
        match self {
            Token::OpenRoundBracket => Token::CloseRoundBracket,
            Token::OpenSquareBracket => Token::CloseSquareBracket,
            _ => panic!("Not an opening bracket"),
        }
    }

    /// Dispatches the appropriate `MolecularFormula` from the current opening
    /// token and the provided formula.
    ///
    /// # Panics
    ///
    /// * If the token is not an opening bracket.
    pub(crate) fn dispatch_wrapped_formula(
        self,
        formula: crate::MolecularFormula,
    ) -> crate::MolecularFormula {
        match self {
            Token::OpenRoundBracket => crate::MolecularFormula::RepeatingUnit(formula.into()),
            Token::OpenSquareBracket => crate::MolecularFormula::Complex(formula.into()),
            _ => panic!("Not an opening bracket"),
        }
    }
}

impl From<Element> for Token {
    fn from(element: Element) -> Self {
        Token::Element(element)
    }
}

impl From<Isotope> for Token {
    fn from(isotope: Isotope) -> Self {
        Token::Isotope(isotope)
    }
}

impl From<GreekLetter> for Token {
    fn from(greek_letter: GreekLetter) -> Self {
        Token::Greek(greek_letter)
    }
}
