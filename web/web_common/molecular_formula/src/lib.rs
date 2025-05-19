#![doc = include_str!("../README.md")]

#[cfg(feature = "pgrx")]
::pgrx::pg_module_magic!();

mod ion;
mod molecular_formula;
pub mod parser;
mod token;
pub use ion::Ion;
pub use molecular_formula::MolecularFormula;
pub mod errors;
