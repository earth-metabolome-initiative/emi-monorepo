//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use std::collections::VecDeque;

use elements_rs::{Element, Isotope};
use num_traits::{CheckedAdd, CheckedMul, ConstOne, ConstZero};

use crate::token::Token;

pub(crate) struct TokenIter<'a> {
    /// The peekable chars iterator
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    /// Tokens already built by failed lookahead attempts.
    tokens: VecDeque<Token>,
}

impl<'a> From<&'a str> for TokenIter<'a> {
    fn from(s: &'a str) -> Self {
        TokenIter { chars: s.chars().peekable(), tokens: VecDeque::new() }
    }
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Returns the digit corresponding to the superscript character.
///
/// # Panics
///
/// * If the character is not a superscript digit.
fn superscript_to_digit(c: char) -> u8 {
    match c {
        '\u{00B9}' => 1,
        '\u{00B2}' => 2,
        '\u{00B3}' => 3,
        '\u{2070}' => 0,
        '\u{2074}' => 4,
        '\u{2075}' => 5,
        '\u{2076}' => 6,
        '\u{2077}' => 7,
        '\u{2078}' => 8,
        '\u{2079}' => 9,
        _ => unreachable!(),
    }
}

/// Returns the digit corresponding to the provided ascii character.
///
/// # Panics
///
/// * If the character is not an ASCII digit.
fn ascii_to_digit(c: char) -> u8 {
    u8::try_from(c.to_digit(10).expect("Expected an ASCII digit")).unwrap()
}

fn is_ascii_minus(c: char) -> bool {
    c == '-' || c == '\u{2212}' // '−'
}

fn is_superscript_plus(c: char) -> bool {
    c == '\u{207A}' // '⁺'
}

fn is_superscript_minus(c: char) -> bool {
    c == '\u{207B}' // '⁻'
}

fn is_superscript_charge(c: char) -> bool {
    is_superscript_plus(c) || is_superscript_minus(c)
}

fn is_ascii_charge(c: char) -> bool {
    is_ascii_minus(c) || c == '+'
}

fn is_any_charge(c: char) -> bool {
    is_superscript_charge(c) || is_ascii_charge(c)
}

fn is_any_plus(c: char) -> bool {
    is_superscript_plus(c) || c == '+'
}

fn is_any_minus(c: char) -> bool {
    is_superscript_minus(c) || is_ascii_minus(c)
}

fn is_dot(c: char) -> bool {
    c == '.' || c == '•' || c == '⋅' || c == '·'
}

impl TokenIter<'_> {
    /// Peaks the next character in the iterator and consumes it if it is a
    /// dot.
    fn consume_dot(&mut self) -> Option<Token> {
        if let Some(&next) = self.chars.peek()
            && is_dot(next)
        {
            self.chars.next();
            return Some(Token::Dot);
        }
        None
    }
    /// Peaks the next character in the iterator and consumes it if it is a
    /// charge.
    fn consume_charge<F>(&mut self, is_charge: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        is_charge(*self.chars.peek()?).then(|| self.chars.next()).flatten()
    }

    fn parse_token(&mut self, current_char: char) -> Result<Token, crate::errors::Error> {
        todo!()
    }
}

impl Iterator for TokenIter<'_> {
    type Item = Result<crate::token::Token, crate::errors::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(token) = self.tokens.pop_front() {
            return Some(Ok(token));
        }

        self.chars.next().map(|current_char| self.parse_token(current_char))
    }
}
