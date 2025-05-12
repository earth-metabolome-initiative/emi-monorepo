//! Submodule providing a `Token` enumeration with the entries which may appear
//! in a molecular formula.

use elements::Element;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents a token in a molecular formula.
pub enum Token {
    /// An element
    Element(Element),
    /// A number
    Number(u16),
    /// A superscript number, which may be an isotope
    Superscript(u16),
    /// A subscript number, which may be a count
    Subscript(u16),
    /// A residual group
    Residual,
    /// A Radical
    Radical,
    /// An open round bracket
    OpenRoundBracket,
    /// A close round bracket
    CloseRoundBracket,
    /// An open square bracket
    OpenSquareBracket,
    /// A close square bracket
    CloseSquareBracket,
    /// A plus sign
    Plus,
    /// A superscript plus sign
    SuperscriptPlus,
    /// A minus sign
    Minus,
    /// A superscript minus sign
    SuperscriptMinus,
    /// A dot
    Dot,
}

impl Token {
    /// Returns whether the token represents a charge.
    pub fn is_charge(&self) -> bool {
        matches!(
            self,
            Token::SuperscriptPlus | Token::SuperscriptMinus | Token::Plus | Token::Minus
        )
    }

    /// Returns the associated closing token for the given opening token.
    ///
    /// # Panics
    ///
    /// * If the token is not an opening bracket.
    pub(crate) fn closing_token(&self) -> Token {
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
        &self,
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
