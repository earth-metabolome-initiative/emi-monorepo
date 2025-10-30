//! Submodule defining traits to create the `diesel` schema from a SQL schema.

mod column_schema;
mod foreign_key_schema;
mod table_schema;
pub use column_schema::ColumnSchema;
pub use foreign_key_schema::ForeignKeySchema;
pub use table_schema::{TABLE_SCHEMA_MODULE_NAME, TableSchema};
