//! Submodule implementing the `FromStr` trait for the `MolecularFormula` struct

use std::str::FromStr;

use super::MolecularFormula;
use crate::parser;

impl FromStr for MolecularFormula {
    type Err = crate::errors::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parser::Parser::from(s).parse()
    }
}
