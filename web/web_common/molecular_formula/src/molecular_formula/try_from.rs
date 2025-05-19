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
        match token {
            Token::Element(element) => Ok(crate::MolecularFormula::Element(element)),
            Token::Residual => Ok(crate::MolecularFormula::Residual),
            Token::CloseRoundBracket
            | Token::CloseSquareBracket
            | Token::Dot
            | Token::Minus
            | Token::Number(_)
            | Token::Radical
            | Token::OpenRoundBracket
            | Token::OpenSquareBracket
            | Token::Plus
            | Token::Superscript(_)
            | Token::Subscript(_)
            | Token::SuperscriptMinus
            | Token::SuperscriptPlus => unreachable!("Invalid token for MolecularFormula"),
        }
    }
}
