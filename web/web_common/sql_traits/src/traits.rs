//! Submodule providing traits for describing SQL-like entities.

pub mod column;
pub mod database;
pub mod table;
pub use column::ColumnLike;
pub use database::DatabaseLike;
pub use table::TableLike;
