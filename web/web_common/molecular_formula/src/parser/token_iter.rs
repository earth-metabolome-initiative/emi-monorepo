//! Submodule creating the `TokenIter` struct, which is an iterator over
//! the `Token`s found in a provided string.

use elements::Element;

pub struct TokenIter<'a> {
    /// The peekable chars iterator
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> From<&'a str> for TokenIter<'a> {
    fn from(s: &'a str) -> Self {
        TokenIter { chars: s.chars().peekable() }
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

impl Iterator for TokenIter<'_> {
    type Item = Result<crate::token::Token, crate::errors::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(c) = self.chars.next() {
            match c {
                '(' => return Some(Ok(crate::token::Token::OpenRoundBracket)),
                ')' => return Some(Ok(crate::token::Token::CloseRoundBracket)),
                '[' => return Some(Ok(crate::token::Token::OpenSquareBracket)),
                ']' => return Some(Ok(crate::token::Token::CloseSquareBracket)),
                '+' => return Some(Ok(crate::token::Token::Plus)),
                '-' => return Some(Ok(crate::token::Token::Minus)),
                '.' => return Some(Ok(crate::token::Token::Dot)),
                '\u{207A}' => return Some(Ok(crate::token::Token::SuperscriptPlus)),
                '\u{207B}' => return Some(Ok(crate::token::Token::SuperscriptMinus)),
                _ if is_superscript_digit(c) => {
                    let number = superscript_to_digit(c);
                    let mut number = match u16::try_from(number) {
                        Ok(number) => number,
                        Err(_) => return Some(Err(crate::errors::Error::InvalidNumber)),
                    };
                    while let Some(&next) = self.chars.peek() {
                        if is_superscript_digit(next) {
                            let next = superscript_to_digit(next);
                            number = number * 10
                                + match u16::try_from(next) {
                                    Ok(number) => number,
                                    Err(_) => {
                                        return Some(Err(crate::errors::Error::InvalidNumber));
                                    }
                                };
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Ok(crate::token::Token::Superscript(number)));
                }
                _ if c.is_ascii_digit() => {
                    let number = c.to_digit(10).unwrap();
                    let mut number = match u16::try_from(number) {
                        Ok(number) => number,
                        Err(_) => return Some(Err(crate::errors::Error::InvalidNumber)),
                    };
                    while let Some(&next) = self.chars.peek() {
                        if next.is_ascii_digit() {
                            let next = next.to_digit(10).unwrap();
                            number = number * 10
                                + match u16::try_from(next) {
                                    Ok(number) => number,
                                    Err(_) => {
                                        return Some(Err(crate::errors::Error::InvalidNumber));
                                    }
                                };
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Ok(crate::token::Token::Number(number)));
                }
                _ if c.is_alphabetic() => {
                    // We peak whether the next character is an alphabetic character
                    // and whether it is lowercase or uppercase.
                    if let Some(&next) = self.chars.peek() {
                        if next.is_lowercase() {
                            self.chars.next();
                            return Some(
                                Element::try_from([c, next]).map(Into::into).map_err(Into::into),
                            );
                        }
                        if c == 'R' {
                            return Some(Ok(crate::token::Token::Residual));
                        }

                        return Some(Element::try_from(c).map(Into::into).map_err(Into::into));
                    }
                    if c == 'R' {
                        return Some(Ok(crate::token::Token::Residual));
                    }
                    return Some(Element::try_from(c).map(Into::into).map_err(Into::into));
                }
                invalid_character => {
                    return Some(Err(crate::errors::Error::InvalidCharacter(invalid_character)));
                }
            }
        }
        None
    }
}
