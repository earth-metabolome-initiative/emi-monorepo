//! Submodule providing a `Token` enumeration with the entries which may appear
//! in a molecular formula.

use elements_rs::{Element, Isotope};
use greek_letters::GreekLetter;
pub mod greek_letters;

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Atom<E> {
    /// Element or isotope
    entity: E,
    /// Whether it is lowercase
    lowercase: bool,
}

impl<E> Atom<E> {
    /// Creates a new `Atom` instance with the given entity and lowercase flag.
    ///
    /// # Arguments
    ///
    /// * `entity` - The element or isotope.
    /// * `lowercase` - Whether the atom is lowercase.
    pub fn new(entity: E, lowercase: bool) -> Self {
        Atom { entity, lowercase }
    }

    /// Returns whether the atom is lowercase.
    pub fn is_lowercase(&self) -> bool {
        self.lowercase
    }
}

impl<E> AsRef<E> for Atom<E> {
    fn as_ref(&self) -> &E {
        &self.entity
    }
}

impl From<Atom<Element>> for Token {
    fn from(atom: Atom<Element>) -> Self {
        Token::Element(atom)
    }
}

impl From<Atom<Isotope>> for Token {
    fn from(atom: Atom<Isotope>) -> Self {
        Token::Isotope(atom)
    }
}

impl From<Atom<Element>> for Element {
    fn from(atom: Atom<Element>) -> Self {
        atom.entity
    }
}

impl From<Atom<Isotope>> for Isotope {
    fn from(atom: Atom<Isotope>) -> Self {
        atom.entity
    }
}

impl From<Element> for Atom<Element> {
    fn from(element: Element) -> Self {
        Atom::new(element, false)
    }
}

impl From<Isotope> for Atom<Isotope> {
    fn from(isotope: Isotope) -> Self {
        Atom::new(isotope, false)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Represents a token in a molecular formula.
pub enum Token {
    /// An element
    Element(Atom<Element>),
    /// An isotope
    Isotope(Atom<Isotope>),
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
        Token::Element(Atom::new(element, false))
    }
}

impl From<Isotope> for Token {
    fn from(isotope: Isotope) -> Self {
        Token::Isotope(Atom::new(isotope, false))
    }
}

impl From<GreekLetter> for Token {
    fn from(greek_letter: GreekLetter) -> Self {
        Token::Greek(greek_letter)
    }
}

impl From<Atom<Element>> for crate::MolecularFormula {
    fn from(atom: Atom<Element>) -> Self {
        crate::MolecularFormula::Element(atom)
    }
}

impl From<Atom<Isotope>> for crate::MolecularFormula {
    fn from(atom: Atom<Isotope>) -> Self {
        crate::MolecularFormula::Isotope(atom)
    }
}
