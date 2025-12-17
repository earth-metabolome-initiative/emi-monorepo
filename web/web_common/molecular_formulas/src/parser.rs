//! Submodule providing a parser for the `MolecularFormula` struct

use std::iter::Peekable;

use crate::{Ion, MolecularFormula, molecular_formula::Side, token::Token};

mod token_iter;
pub use token_iter::TokenIter;

/// Parser for the `MolecularFormula` struct
pub struct Parser<I: Iterator<Item = char>> {
    tokens_iter: Peekable<token_iter::TokenIter<I>>,
}

impl<'a> From<&'a str> for Parser<std::str::Chars<'a>> {
    fn from(s: &'a str) -> Self {
        let tokens_iter = token_iter::TokenIter::from(s).peekable();
        Parser { tokens_iter }
    }
}

impl<I: Iterator<Item = char>> From<I> for Parser<I> {
    fn from(iter: I) -> Self {
        let tokens_iter = token_iter::TokenIter::from(iter).peekable();
        Parser { tokens_iter }
    }
}

impl<I: Iterator<Item = char>> From<TokenIter<I>> for Parser<I> {
    fn from(tokens_iter: TokenIter<I>) -> Self {
        Parser { tokens_iter: tokens_iter.peekable() }
    }
}

impl<I: Iterator<Item = char>> Parser<I> {
    #[allow(clippy::too_many_lines)]
    fn update_formula(
        &mut self,
        formula: Option<MolecularFormula>,
        token: Token,
    ) -> Result<Option<MolecularFormula>, crate::errors::Error> {
        let new_formula = match (token, formula) {
            (Token::Residual | Token::Element(_) | Token::Isotope(_), previous) => {
                Some(match previous {
                    Some(previous) => previous.chain(token.into()),
                    None => token.into(),
                })
            }
            (Token::Greek(greek_letter), None) => {
                let (inner_formula, closing_token) = self.inner_parse()?;
                if closing_token.is_some() {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
                match inner_formula {
                    MolecularFormula::Sequence(mut sequence) => {
                        sequence.insert(0, MolecularFormula::Greek(greek_letter));
                        Some(MolecularFormula::Sequence(sequence))
                    }
                    other => Some(MolecularFormula::Greek(greek_letter).chain(other)),
                }
            }
            (Token::Greek(greek_letter), Some(_)) => {
                return Err(crate::errors::Error::InvalidGreekLetterPosition(greek_letter));
            }
            (Token::Count(count), None) => {
                let (inner_formula, closing_token) = self.inner_parse()?;
                if closing_token.is_some() {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
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
                    _ => Some(inner_formula.add_count_to_first_subformula(count)?),
                }
            }
            (Token::Count(count), Some(formula)) => {
                Some(formula.add_count_to_last_subformula(count)?)
            }
            (Token::Charge(_), Some(MolecularFormula::Ion(_)) | None) => {
                return Err(crate::errors::Error::InvalidChargePosition);
            }
            (Token::Charge(charge), Some(formula)) => {
                Some(Ion::from_formula(formula, charge)?.into())
            }
            (Token::Dot, formula) => {
                let (inner_formula, closing_token) = match (formula.is_some(), self.inner_parse()) {
                    (_, Ok(result)) => result,
                    (true, Err(crate::errors::Error::EmptyFormula)) => {
                        return Ok(Some(MolecularFormula::Radical(
                            formula.unwrap().into(),
                            Side::Right,
                        )));
                    }
                    (_, Err(e)) => return Err(e),
                };
                if closing_token.is_some() {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }

                Some(match formula {
                    Some(MolecularFormula::Mixture(mut mixture)) => {
                        mixture.push(inner_formula);
                        MolecularFormula::Mixture(mixture)
                    }
                    Some(other) => MolecularFormula::Mixture(vec![other, inner_formula]),
                    None => MolecularFormula::Radical(inner_formula.into(), Side::Left),
                })
            }
            (Token::OpenRoundBracket | Token::OpenSquareBracket, outer_formula) => {
                let (inner_formula, closing_token) = self.inner_parse()?;
                let expected_closing_token = token.closing_token();
                if closing_token != Some(expected_closing_token) {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: Some(expected_closing_token),
                        found: closing_token,
                    });
                }
                let inner_formula = token.dispatch_wrapped_formula(inner_formula);

                match outer_formula {
                    None => Some(inner_formula),
                    Some(outer_formula) => Some(outer_formula.chain(inner_formula)),
                }
            }
            (_, Some(MolecularFormula::Sequence(mut sequence))) => {
                let last = sequence.last().unwrap().clone();
                let new_formula = self.update_formula(Some(last), token)?;
                let number_of_elements = sequence.len();
                sequence[number_of_elements - 1] = new_formula.unwrap();
                Some(MolecularFormula::Sequence(sequence))
            }
            (Token::CloseRoundBracket | Token::CloseSquareBracket, _) => {
                unreachable!("This case should be handled in the `inner_parse` function")
            }
        };

        Ok(new_formula)
    }

    fn inner_parse(&mut self) -> Result<(MolecularFormula, Option<Token>), crate::errors::Error> {
        let mut formula = None;
        while let Some(token) = self.tokens_iter.next().transpose()? {
            if matches!(token, Token::CloseRoundBracket | Token::CloseSquareBracket) {
                return if let Some(formula) = formula {
                    Ok((formula, Some(token)))
                } else {
                    Err(crate::errors::Error::EmptyFormula)
                };
            }

            let new_formula = self.update_formula(formula, token)?;
            formula = new_formula;
        }

        if let Some(formula) = formula {
            Ok((formula, None))
        } else {
            Err(crate::errors::Error::EmptyFormula)
        }
    }

    pub(crate) fn parse(mut self) -> Result<MolecularFormula, crate::errors::Error> {
        let (formula, token) = self.inner_parse()?;
        if token.is_some() {
            return Err(crate::errors::Error::ClosingToken { expected: None, found: token });
        }
        if self.tokens_iter.peek().is_some() {
            return Err(crate::errors::Error::UnconsumedParser);
        }
        Ok(formula)
    }
}
