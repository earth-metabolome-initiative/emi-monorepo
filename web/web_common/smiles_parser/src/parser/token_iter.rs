//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use elements_rs::{Element, Isotope};
use molecular_formulas::MolecularFormula;
use molecular_formulas::Token as MolecularFormulaToken;
use molecular_formulas::parser::TokenIter as MolecularFormulaTokenIter;

use crate::token::Token;

/// An iterator over the tokens found in a SMILES string.
pub struct TokenIter<'a> {
    /// The peekable chars iterator
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> From<&'a str> for TokenIter<'a> {
    fn from(s: &'a str) -> Self {
        TokenIter { chars: s.chars().peekable() }
    }
}

impl TokenIter<'_> {
    fn parse_token(&mut self, current_char: char) -> Result<Token, crate::errors::Error> {
        Ok(match current_char {
            '(' => Token::OpenRoundBracket,
            ')' => Token::CloseRoundBracket,
            '=' => Token::Equal,
            '#' => Token::Hashtag,
            '$' => Token::Dollar,
            '.' => Token::Dot,
            ':' => Token::Colon,
            '/' => Token::ForwardSlash,
            '\\' => Token::BackSlash,
            '[' => {
                let molecule_iter = self.chars.by_ref().take_while(|c| *c != ']');
                let mut molecule_token_iter = MolecularFormulaTokenIter::from(molecule_iter);
                if let Some(parsed) = molecule_token_iter.next().transpose()? {
                    if matches!(
                        parsed,
                        MolecularFormulaToken::Element(_) | MolecularFormulaToken::Isotope(_)
                    ) {
                        // handle chirality tokens
                        let chars = molecule_token_iter.chars();
                        if let Some(&'@') = chars.peek() {
                            chars.next();
                            if let Some(&'@') = chars.peek() {
                                chars.next();
                                let chirality_token = MolecularFormulaToken::ChiralityClockwise;
                                molecule_token_iter.push_back(chirality_token);
                            } else {
                                let chirality_token =
                                    MolecularFormulaToken::ChiralityCounterClockwise;
                                molecule_token_iter.push_back(chirality_token);
                            }
                        }
                    }
                    molecule_token_iter.push_back(parsed)
                }
                let molecule = MolecularFormula::try_from_token_iter(molecule_token_iter)?;
                Token::MolecularFormula(molecule)
            }
            maybe_number @ '0'..='9' => {
                let label = u8::try_from(maybe_number.to_digit(10).unwrap()).unwrap();
                Token::Label(label)
            }
            maybe_element_char => {
                if maybe_element_char.is_lowercase()
                    && let Ok(element) = Element::try_from(maybe_element_char)
                {
                    return Ok(Token::AromaticMolecularFormula(element.into()));
                }
                if let Some(next_char) = self.chars.peek()
                    && let Ok(element) = Element::try_from([maybe_element_char, *next_char])
                {
                    self.chars.next();
                    return Ok(Token::MolecularFormula(element.into()));
                }
                if let Ok(element) = Element::try_from(maybe_element_char) {
                    return Ok(Token::MolecularFormula(element.into()));
                }
                return Err(crate::errors::Error::UnexpectedCharacter {
                    character: maybe_element_char,
                });
            }
        })
    }
}

impl Iterator for TokenIter<'_> {
    type Item = Result<crate::token::Token, crate::errors::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        self.chars.next().map(|current_char| self.parse_token(current_char))
    }
}
