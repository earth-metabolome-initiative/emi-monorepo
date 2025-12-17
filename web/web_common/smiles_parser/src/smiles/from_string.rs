use std::str::FromStr;

use super::Smiles;
use crate::{parser::token_iter::TokenIter, token::Token};

impl FromStr for Smiles {
    type Err = crate::errors::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token_iter = TokenIter::from(s);
        let tokens = token_iter.collect::<Result<Vec<_>, _>>()?;
        todo!()
    }
}
