//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use std::collections::VecDeque;

use elements::{Element, Isotope};
use num_traits::{CheckedAdd, CheckedMul, ConstOne, ConstZero};

use crate::token::{Token, greek_letters::GreekLetter};

pub struct TokenIter<'a> {
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

fn is_superscript_digit(c: char) -> bool {
    matches!(
        c,
        '\u{00B9}' // ¹
        | '\u{00B2}' // ²
        | '\u{00B3}' // ³
        | '\u{2070}' // ⁰
        | '\u{2074}'..='\u{2079}' // ⁴ to ⁹
    )
}

fn is_ascii_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_subscript_digit(c: char) -> bool {
    matches!(
        c,
        '\u{2080}' // ₀
        | '\u{2081}' // ₁
        | '\u{2082}' // ₂
        | '\u{2083}' // ₃
        | '\u{2084}' // ₄
        | '\u{2085}' // ₅
        | '\u{2086}' // ₆
        | '\u{2087}' // ₇
        | '\u{2088}' // ₈
        | '\u{2089}' // ₉
    )
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

/// Returns the digit corresponding to the subscript character.
///
/// # Panics
///
/// * If the character is not a subscript digit.
fn subscript_to_digit<T: From<u8>>(c: char) -> T {
    T::from(match c {
        '\u{2080}' => 0u8,
        '\u{2081}' => 1u8,
        '\u{2082}' => 2u8,
        '\u{2083}' => 3u8,
        '\u{2084}' => 4u8,
        '\u{2085}' => 5u8,
        '\u{2086}' => 6u8,
        '\u{2087}' => 7u8,
        '\u{2088}' => 8u8,
        '\u{2089}' => 9u8,
        _ => unreachable!(),
    })
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
    fn consume_digit<T: From<u8> + ConstOne + CheckedMul + CheckedAdd>(
        &mut self,
        starting_digit: char,
        to_digit: fn(char) -> u8,
        is_digit: fn(char) -> bool,
    ) -> Result<T, crate::errors::Error> {
        let mut number: T = T::from(to_digit(starting_digit));
        let ten =
            T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE + T::ONE; // 10
        while let Some(&next) = self.chars.peek() {
            if is_digit(next) {
                let next = to_digit(next);
                number = number.checked_mul(&ten).ok_or(crate::errors::Error::InvalidNumber)?;
                number = number
                    .checked_add(&T::from(next))
                    .ok_or(crate::errors::Error::InvalidNumber)?;
                self.chars.next();
            } else {
                break;
            }
        }
        Ok(number)
    }

    fn maybe_consume_digit<T: From<u8> + ConstZero + ConstOne + CheckedMul + CheckedAdd>(
        &mut self,
        to_digit: fn(char) -> u8,
        is_digit: fn(char) -> bool,
    ) -> Result<Option<T>, crate::errors::Error> {
        Ok(if let Some(&next) = self.chars.peek() {
            if is_digit(next) {
                self.chars.next();
                Some(self.consume_digit(next, to_digit, is_digit)?)
            } else {
                None
            }
        } else {
            None
        })
    }

    /// Peaks the next character in the iterator and consumes it if it is an
    /// ASCII minus sign.
    fn consume_minus(&mut self) -> Option<char> {
        is_ascii_minus(*self.chars.peek()?).then(|| self.chars.next()).flatten()
    }

    /// Peaks the next character in the iterator and consumes it if it is a
    /// charge.
    fn consume_charge<F>(&mut self, is_charge: F) -> Option<char>
    where
        F: Fn(char) -> bool,
    {
        is_charge(*self.chars.peek()?).then(|| self.chars.next()).flatten()
    }

    /// Peaks the next alphabetical character in the iterator and consumes it
    /// if it is a solely alphabetical character.
    fn consume_alphabetic(&mut self, char: char) -> Result<Token, crate::errors::Error> {
        assert!(char.is_alphabetic() && char.is_uppercase());

        if let Some(&next) = self.chars.peek()
            && next.is_lowercase()
            && next.is_ascii_alphabetic()
        {
            self.chars.next();
            return Element::try_from([char, next]).map(Into::into).map_err(Into::into);
        }
        // To handle cases like 'P', 'D' and 'T', which respectively
        // represent Protium, Deuterium and Tritium.
        if let Ok(isotope) = Isotope::try_from(char) {
            Ok(crate::token::Token::Isotope(isotope))
        } else if char == 'R' {
            Ok(crate::token::Token::Residual)
        } else {
            Element::try_from(char).map(Into::into).map_err(Into::into)
        }
    }

    /// Peaks the next character in the iterator and consumes it if it is an
    /// element. This operation requires us to double lookahead, as we need to
    /// check whether the next character is a a lowercase letter, which may
    /// indicate a two-letter element. This operation may fail, and in that case
    /// we may need to push the newly built token back to the `tokens` queue.
    fn consume_element(&mut self) -> Option<Element> {
        if let Some(&next) = self.chars.peek()
            && next.is_ascii_alphabetic()
            && next.is_uppercase()
        {
            // If the next character is not lowercase, we consume only the first one.
            let char = self.chars.next().unwrap();
            if let Ok(token) = self.consume_alphabetic(char) {
                // If the token is an element, we return it.
                if let Token::Element(element) = token {
                    return Some(element);
                }
                // If the token is not an element, we push it back to the queue.
                self.tokens.push_back(token);
            }
        }
        None
    }

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

    fn parse_token(&mut self, current_char: char) -> Result<Token, crate::errors::Error> {
        Ok(match current_char {
            '(' => crate::token::Token::OpenRoundBracket,
            ')' => crate::token::Token::CloseRoundBracket,
            '[' => crate::token::Token::OpenSquareBracket,
            ']' => crate::token::Token::CloseSquareBracket,
            maybe_dot if is_dot(maybe_dot) => {
                // We peak that the next character is not a dot, or we
                // raise a `InvalidRepeatedToken` error.
                if self.consume_dot().is_some() {
                    return Err(crate::errors::Error::InvalidRepeatedToken(
                        crate::token::Token::Dot,
                    ));
                }

                crate::token::Token::Dot
            }
            c if GreekLetter::is_greek_letter(c) => {
                // We peak whether the next character is a 'Token::Minus',
                // which is necessary to determine whether the greek letter is
                // in a valid position or it is just appearing in the middle of a
                // formula. We will need to validate in the parser that the previous
                // token is consistent with the greek letter.
                let greek_letter = GreekLetter::try_from(c).unwrap();
                if self.consume_minus().is_some() {
                    // We can unwrap as we know that the character is a valid greek letter.
                    greek_letter.into()
                } else {
                    // If the next character is not a minus sign.
                    return Err(crate::errors::Error::InvalidGreekLetterPosition(greek_letter));
                }
            }
            maybe_charge if is_any_charge(maybe_charge) => {
                // If the next character is a superscript digit, we proceed
                // to fold it into a charge.
                let mut charge = if is_any_plus(maybe_charge) { 1 } else { -1 };

                if let Some(digit) = self.maybe_consume_digit(
                    if is_superscript_charge(maybe_charge) {
                        superscript_to_digit
                    } else {
                        ascii_to_digit
                    },
                    if is_superscript_charge(maybe_charge) {
                        is_superscript_digit
                    } else {
                        is_ascii_digit
                    },
                )? {
                    charge =
                        charge.checked_mul(&digit).ok_or(crate::errors::Error::InvalidNumber)?;
                }

                if charge == 0 {
                    return Err(crate::errors::Error::ZeroCharge);
                }

                crate::token::Token::Charge(charge)
            }
            maybe_ascii_digit if maybe_ascii_digit.is_ascii_digit() => {
                let digit =
                    self.consume_digit(maybe_ascii_digit, ascii_to_digit, is_ascii_digit)?;

                if digit == 0 {
                    return Err(crate::errors::Error::ZeroCount);
                }

                Token::Count(digit)
            }
            maybe_superscript_digit if is_superscript_digit(maybe_superscript_digit) => {
                let digit: u16 = self.consume_digit(
                    maybe_superscript_digit,
                    superscript_to_digit,
                    is_superscript_digit,
                )?;

                // We check whether the subsequent character is a charge.
                if let Some(charge) =
                    self.consume_charge(if is_superscript_digit(maybe_superscript_digit) {
                        is_superscript_charge
                    } else {
                        is_ascii_charge
                    })
                {
                    if digit == 0 {
                        return Err(crate::errors::Error::ZeroCharge);
                    }
                    let mut signed_digit = i16::try_from(digit)?;
                    if is_any_minus(charge) {
                        signed_digit = -signed_digit;
                    }

                    return Ok(Token::Charge(signed_digit));
                }
                // If this is not a charge, and it is followed by an
                // element, this scalar may be meant as
                // the mass number of an isotope.
                else if let Some(element) = self.consume_element()
                    && let Ok(isotope) = Isotope::try_from((element, digit))
                {
                    return Ok(Token::Isotope(isotope));
                }

                return Err(crate::errors::Error::InvalidSuperscriptPosition);
            }
            maybe_subscript_digit if is_subscript_digit(maybe_subscript_digit) => {
                let digit = self.consume_digit(
                    maybe_subscript_digit,
                    subscript_to_digit,
                    is_subscript_digit,
                )?;

                if digit == 0 {
                    return Err(crate::errors::Error::ZeroCount);
                }

                // A subscript is always a count, so we return it as such.
                crate::token::Token::Count(digit)
            }
            c if c.is_ascii_alphabetic() && c.is_uppercase() => self.consume_alphabetic(c)?,
            invalid_character => {
                return Err(crate::errors::Error::InvalidCharacter(invalid_character));
            }
        })
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
