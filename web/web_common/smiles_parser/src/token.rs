use elements_rs::{Element, Isotope};

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
/// Represents a token in a molecular formula.
pub(crate) enum Token {
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
    /// A dash '-' character
    Dash,
    /// An equal '=' character
    Equal,
    /// A hash '#' character
    Hash,
    /// A colon ':' character
    Colon,
    /// A foward slash '/' character
    ForwardSlash,
    /// A back slash '\' character
    BackSlash,
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
