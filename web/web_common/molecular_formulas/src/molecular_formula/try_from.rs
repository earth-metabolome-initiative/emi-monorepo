//! Submodule implementing the `TryFrom` trait for the `MolecularFormula` struct

use crate::{parser::TokenIter, token::Token};

impl TryFrom<&str> for crate::MolecularFormula {
    type Error = crate::errors::Error;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(s)
    }
}

impl TryFrom<String> for crate::MolecularFormula {
    type Error = crate::errors::Error;

    #[inline]
    fn try_from(s: String) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(&s)
    }
}

impl TryFrom<&String> for crate::MolecularFormula {
    type Error = crate::errors::Error;

    #[inline]
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(s)
    }
}

impl crate::MolecularFormula {
    /// Tries to parse a `MolecularFormula` from an iterator of characters.
    ///
    /// # Arguments
    ///
    /// * `iter` - An iterator of characters representing the molecular formula.
    ///
    /// # Errors
    ///
    /// Returns an error if the parsing fails.
    #[inline]
    pub fn try_from_iter<I: Iterator<Item = char>>(iter: I) -> Result<Self, crate::errors::Error> {
        crate::parser::Parser::from(iter).parse()
    }

    /// Tries to parse a `MolecularFormula` from a `TokenIter`.
    ///
    /// # Arguments
    ///
    /// * `token_iter` - A `TokenIter` representing the molecular formula.
    ///
    /// # Errors
    ///
    /// Returns an error if the parsing fails.
    #[inline]
    pub fn try_from_token_iter<I: Iterator<Item = char>>(
        token_iter: TokenIter<I>,
    ) -> Result<Self, crate::errors::Error> {
        crate::parser::Parser::from(token_iter).parse()
    }
}

impl From<Token> for crate::MolecularFormula {
    fn from(token: Token) -> Self {
        match token {
            Token::Element(element) => element.into(),
            Token::Isotope(isotope) => isotope.into(),
            Token::Greek(greek_letter) => greek_letter.into(),
            Token::Residual => crate::MolecularFormula::Residual,
            Token::CloseRoundBracket
            | Token::CloseSquareBracket
            | Token::Dot
            | Token::OpenRoundBracket
            | Token::Charge(_)
            | Token::Count(_)
            | Token::OpenSquareBracket => unreachable!("Invalid token for MolecularFormula"),
        }
    }
}
