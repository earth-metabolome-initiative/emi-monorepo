#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![deny(unused_macro_rules)]
#![deny(unconditional_recursion)]
#![deny(unreachable_patterns)]
#![deny(unused_import_braces)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod csv_columns;
mod csv_schema;
mod csv_table;
mod data_types;
mod errors;
mod metadata;
pub mod extensions;

pub use csv_schema::{CSVSchema, CSVSchemaBuilder};
pub use csv_table::CSVTable;
pub use csv_columns::CSVColumn;
pub use errors::CSVSchemaError;
