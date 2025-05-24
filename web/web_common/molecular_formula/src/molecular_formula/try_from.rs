//! Submodule implementing the `TryFrom` trait for the `MolecularFormula` struct

use std::convert::Infallible;

use crate::token::Token;

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

impl TryFrom<Token> for crate::MolecularFormula {
    type Error = Infallible;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        Ok(match token {
            Token::Element(element) => element.into(),
            Token::Isotope(isotope) => isotope.into(),
            Token::Greek(greek_letter) => greek_letter.into(),
            Token::Residual => crate::MolecularFormula::Residual,
            Token::CloseRoundBracket
            | Token::CloseSquareBracket
            | Token::Dot
            | Token::Radical
            | Token::OpenRoundBracket
            | Token::Charge(_)
            | Token::Count(_)
            | Token::OpenSquareBracket => unreachable!("Invalid token for MolecularFormula"),
        })
    }
}
