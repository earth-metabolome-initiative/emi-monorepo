#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod csv_table;
mod csv_columns;
mod csv_schema;
mod data_types;
mod errors;
mod metadata;

pub use csv_schema::{CSVSchema, CSVSchemaBuilder};