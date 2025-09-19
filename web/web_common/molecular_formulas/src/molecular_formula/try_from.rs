//! Submodule implementing the `TryFrom` trait for the `MolecularFormula` struct

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

impl TryFrom<&String> for crate::MolecularFormula {
    type Error = crate::errors::Error;

    #[inline]
    fn try_from(s: &String) -> Result<Self, Self::Error> {
        <crate::MolecularFormula as std::str::FromStr>::from_str(s)
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
