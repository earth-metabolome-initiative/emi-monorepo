#![doc = include_str!("../README.md")]

mod ion;
mod molecular_formula;
pub mod parser;
mod solvation;
mod token;
pub use ion::Ion;
pub use molecular_formula::MolecularFormula;
pub use solvation::Solvation;
pub mod errors;
