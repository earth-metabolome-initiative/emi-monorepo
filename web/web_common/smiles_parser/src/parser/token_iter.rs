//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use std::collections::VecDeque;

use elements_rs::{Element, Isotope};
use num_traits::{CheckedAdd, CheckedMul, ConstOne, ConstZero};

use crate::token::Token;

pub(crate) struct TokenIter<'a> {
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
            maybe_element_char => {}
            unexpected_char => {
                return Err(crate::errors::Error::UnexpectedCharacter {
                    character: unexpected_char,
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
