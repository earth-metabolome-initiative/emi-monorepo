//! Submodule defining traits to generate `quote` tokens for SQL constructs.

mod table;
pub use table::TableSynLike;
mod column;
pub use column::ColumnSynLike;
mod external_dependencies;
pub use external_dependencies::ExternalDependencies;
mod internal_dependencies;
pub use internal_dependencies::InternalDependencies;
mod foreign_key;
pub use foreign_key::ForeignKeySynLike;
mod function;
pub use function::FunctionSynLike;
