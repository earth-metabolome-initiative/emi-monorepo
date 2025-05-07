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
                '*' => return Some(Ok(crate::token::Token::Mul)),
                _ if c.is_ascii_digit() => {
                    let Some(number) = c.to_digit(10) else {
                        return Some(Err(crate::errors::Error::InvalidNumber));
                    };
                    let mut number = match u8::try_from(number) {
                        Ok(number) => number,
                        Err(_) => return Some(Err(crate::errors::Error::InvalidNumber)),
                    };
                    while let Some(&next) = self.chars.peek() {
                        if next.is_ascii_digit() {
                            let Some(next) = next.to_digit(10) else {
                                return Some(Err(crate::errors::Error::InvalidNumber));
                            };
                            number = number * 10
                                + match u8::try_from(next) {
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
                    return Some(Ok(number.into()));
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
                        } else {
                            return Some(Element::try_from(c).map(Into::into).map_err(Into::into));
                        }
                    } else {
                        return Some(Element::try_from(c).map(Into::into).map_err(Into::into));
                    }
                }
                _ => {}
            }
        }
        None
    }
}
