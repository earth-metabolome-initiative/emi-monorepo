//! Submodule defining traits to create the `diesel` schema from a SQL schema.

mod table_schema;
pub use table_schema::TableSchema;
