//! Submodule providing a parser for the `MolecularFormula` struct

use std::iter::Peekable;

use crate::{Ion, MolecularFormula, Solvation, token::Token};

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

impl<'a> Parser<'a> {
    fn update_formula(
        mut self,
        formula: Option<MolecularFormula>,
        token: Token,
    ) -> Result<(Self, Option<MolecularFormula>), crate::errors::Error> {
        let new_formula = match (token, formula) {
            (Token::Element(element), None) => Some(element.into()),
            (Token::Element(element), Some(previous)) => Some(previous.chain(element.into())),
            (Token::Number(count), None) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token != None {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
                self = parser;
                match inner_formula {
                    MolecularFormula::Sequence(sequence) => {
                        Some(MolecularFormula::Count(
                            MolecularFormula::RepeatingUnit(sequence.into()).into(),
                            count,
                        ))
                    }
                    _ => inner_formula.add_count_to_first_subformula(count).ok(),
                }
            }
            (Token::Plus, Some(MolecularFormula::Element(element))) => {
                Some(MolecularFormula::Ion(Ion::new(element.into(), 1)))
            }
            (Token::Minus, Some(MolecularFormula::Element(element))) => {
                Some(MolecularFormula::Ion(Ion::new(element.into(), -1)))
            }
            (Token::Mul, Some(formula)) => {
                let (parser, inner_formula, closing_token) = self.inner_parse()?;
                if closing_token != None {
                    return Err(crate::errors::Error::ClosingToken {
                        expected: None,
                        found: closing_token,
                    });
                }
                self = parser;
                Some(Solvation::new(formula.into(), inner_formula.into()).into())
            }
            (Token::Plus, Some(MolecularFormula::Count(formula, count))) => {
                match *formula {
                    MolecularFormula::Element(element) => {
                        Some(MolecularFormula::Ion(Ion::new(
                            element.into(),
                            i8::try_from(count).map_err(|_| crate::errors::Error::InvalidNumber)?,
                        )))
                    }
                    formula => {
                        Some(MolecularFormula::Ion(Ion::new(
                            MolecularFormula::Count(formula.into(), count).into(),
                            1,
                        )))
                    }
                }
            }
            (Token::Minus, Some(MolecularFormula::Count(formula, count))) => {
                match *formula {
                    MolecularFormula::Element(element) => {
                        Some(MolecularFormula::Ion(Ion::new(
                            element.into(),
                            -i8::try_from(count)
                                .map_err(|_| crate::errors::Error::InvalidNumber)?,
                        )))
                    }
                    formula => {
                        Some(MolecularFormula::Ion(Ion::new(
                            MolecularFormula::Count(formula.into(), count).into(),
                            -1,
                        )))
                    }
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
            (Token::Mul | Token::Plus | Token::Minus, None) => {
                return Err(crate::errors::Error::InvalidLeadingToken(token));
            }

            case => unimplemented!("Parsing of the case {case:?} is not implemented"),
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
        if !parser.tokens_iter.peek().is_none() {
            return Err(crate::errors::Error::UnconsumedParser);
        }
        Ok(formula)
    }
}
