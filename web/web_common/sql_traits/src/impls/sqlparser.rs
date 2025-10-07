#![cfg(feature = "sqlparser")]
//! Submodule providing implementations of the traits defined in the `traits`
//! module for the `sqlparser` crate.

mod check_constraint;
mod column_def;
mod create_table;
pub mod database;
mod unique_constraint;
pub use database::SqlParserDatabase;
mod foreign_key_constraint;
