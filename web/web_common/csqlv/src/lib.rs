#![doc = include_str!("../README.md")]

mod csv_columns;
mod csv_schema;
mod csv_table;
mod data_types;
mod errors;
pub mod extensions;
mod metadata;
pub mod sql_generation_options;

pub use csv_columns::CSVColumn;
pub use csv_schema::{CSVSchema, CSVSchemaBuilder};
pub use csv_table::CSVTable;
pub use errors::CSVSchemaError;
pub use sql_generation_options::SQLGenerationOptions;
