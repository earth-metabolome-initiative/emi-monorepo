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
}

impl From<Element> for Token {
    fn from(element: Element) -> Self {
        Token::Element(element)
    }
}
