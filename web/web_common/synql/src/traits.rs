//! Submodule defining traits used to generate Rust code from SQL schema.

pub mod synql_database_like;
pub use synql_database_like::SynQLDatabaseLike;
pub mod table;
pub use table::TableSynLike;
pub mod column;
pub use column::ColumnSynLike;
pub mod check_constraint;
pub use check_constraint::CheckConstraintSynLike;
pub mod function;
pub use function::FunctionSynLike;
