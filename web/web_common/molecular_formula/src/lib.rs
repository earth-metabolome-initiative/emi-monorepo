#![doc = include_str!("../README.md")]

pub mod parser;
mod ion;
mod molecular_formula;
mod solvation;
mod token;
pub use molecular_formula::MolecularFormula;
pub use solvation::Solvation;
pub use ion::Ion;
pub mod errors;
