//! Submodule providing a parser for the `MolecularFormula` struct

use std::iter::Peekable;

use crate::{Ion, MolecularFormula, token::Token};

mod token_iter;

/// Parser for the `MolecularFormula` struct
pub struct Parser<'a> {
    tokens_iter: Peekable<token_iter::TokenIter<'a>>,
}

impl<'a> From<&'a str> for Parser<'a> {
    fn from(s: &'a str) -> Self {
        let tokens_iter = token_iter::TokenIter::from(s).peekable();
        Parser { tokens_iter }
    }
}

impl Parser<'_> {
    #[allow(clippy::too_many_lines)]
    fn update_formula(
        mut self,
        formula: Option<MolecularFormula>,
        token: Token,
    ) -> Result<(Self, Option<MolecularFormula>), crate::errors::Error> {
        let new_formula = match (token, formula) {
            (Token::Element(element), None) => Some(element.into()),
            (Token::Residual, None) => Some(MolecularFormula::Residual),
            (Token::Element(element), Some(previous)) => Some(previous.chain(element.into())),
            (Token::Residual, Some(previous)) => Some(previous.chain(MolecularFormula::Residual)),
            (Token::Number(count), None) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token.is_some() {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
                self = parser;
                match inner_formula {
                    MolecularFormula::Sequence(sequence) => {
                        Some(MolecularFormula::Count(
                            MolecularFormula::RepeatingUnit(Box::new(MolecularFormula::Sequence(
                                sequence,
                            )))
                            .into(),
                            count,
                        ))
                    }
                    _ => inner_formula.add_count_to_first_subformula(count).ok(),
                }
            }
            (Token::Plus | Token::Minus, Some(formula)) => {
                // We need to check whether the next token is a number or not. If
                // it is, we create a new `Ion` with the element and the number,
                // otherwise we create a new `Ion` with the element and charge 1.
                let mut charge: i8 = if let Some(Token::Number(charge)) =
                    self.tokens_iter.peek().copied().transpose()?
                {
                    self.tokens_iter.next();
                    i8::try_from(charge).map_err(|_| crate::errors::Error::InvalidNumber)?
                } else {
                    1
                };

                if token == Token::Minus {
                    charge *= -1;
                }

                Some(Ion::from_formula(formula, charge)?.into())
            }
            (Token::Dot, Some(formula)) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token.is_some() {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
                self = parser;
                if let MolecularFormula::Mixture(mut mixture) = formula {
                    mixture.push(inner_formula);
                    Some(MolecularFormula::Mixture(mixture))
                } else {
                    Some(MolecularFormula::Mixture(vec![formula, inner_formula]))
                }
            }
            (Token::OpenRoundBracket, outer_formula) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token != Some(Token::CloseRoundBracket) {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: Some(Token::CloseRoundBracket),
                        found: closing_token,
                    });
                }
                self = parser;
                let inner_formula = MolecularFormula::RepeatingUnit(inner_formula.into());

                match outer_formula {
                    None => Some(inner_formula),
                    Some(outer_formula) => Some(outer_formula.chain(inner_formula)),
                }
            }

            (Token::OpenSquareBracket, outer_formula) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token != Some(Token::CloseSquareBracket) {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: Some(Token::CloseSquareBracket),
                        found: closing_token,
                    });
                }
                self = parser;
                let inner_formula = MolecularFormula::Complex(inner_formula.into());

                match outer_formula {
                    None => Some(inner_formula),
                    Some(outer_formula) => Some(outer_formula.chain(inner_formula)),
                }
            }
            (_, Some(MolecularFormula::Sequence(mut sequence))) => {
                let last = sequence.last().unwrap().clone();
                let (new_parser, new_formula) = self.update_formula(Some(last), token)?;
                self = new_parser;
                let number_of_elements = sequence.len();
                sequence[number_of_elements - 1] = new_formula.unwrap();
                Some(MolecularFormula::Sequence(sequence))
            }
            (Token::Number(count), Some(formula)) => {
                Some(MolecularFormula::Count(formula.into(), count))
            }
            (Token::CloseRoundBracket | Token::CloseSquareBracket, _) => {
                unreachable!("This case should be handled in the `inner_parse` function")
            }
            (Token::Dot | Token::Plus | Token::Minus, None) => {
                return Err(crate::errors::Error::InvalidLeadingToken(token));
            }
        };

        Ok((self, new_formula))
    }

    fn inner_parse(
        mut self,
    ) -> Result<(Self, MolecularFormula, Option<Token>), crate::errors::Error> {
        let mut formula = None;
        while let Some(token) = self.tokens_iter.next().transpose()? {
            if matches!(token, Token::CloseRoundBracket | Token::CloseSquareBracket) {
                return if let Some(formula) = formula {
                    Ok((self, formula, Some(token)))
                } else {
                    Err(crate::errors::Error::EmptyFormula)
                };
            }

            let (new_parser, new_formula) = self.update_formula(formula, token)?;
            self = new_parser;
            formula = new_formula;
        }

        if let Some(formula) = formula {
            Ok((self, formula, None))
        } else {
            Err(crate::errors::Error::EmptyFormula)
        }
    }

    pub(crate) fn parse(self) -> Result<MolecularFormula, crate::errors::Error> {
        let (mut parser, formula, token) = self.inner_parse()?;
        if token.is_some() {
            return Err(crate::errors::Error::ClosingToken { expected: None, found: token });
        }
        if parser.tokens_iter.peek().is_some() {
            return Err(crate::errors::Error::UnconsumedParser);
        }
        Ok(formula)
    }
}
