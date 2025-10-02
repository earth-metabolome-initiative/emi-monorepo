#![cfg(feature = "sqlparser")]
//! Submodule providing implementations of the traits defined in the `traits`
//! module for the `sqlparser` crate.

mod column_def;
mod create_table;
pub mod database;
pub use database::SqlParserDatabase;