//! Submodule providing a `Token` enumeration with the entries which may appear
//! in a molecular formula.

use elements::Element;

#[derive(Debug, PartialEq, Clone, Copy)]
/// Represents a token in a molecular formula.
pub enum Token {
    /// An element
    Element(Element),
    /// A number
    Number(u8),
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
    /// A minus sign
    Minus,
    /// A dot
    Dot,
}

impl From<u8> for Token {
    fn from(number: u8) -> Self {
        Token::Number(number)
    }
}

impl From<Element> for Token {
    fn from(element: Element) -> Self {
        Token::Element(element)
    }
}
