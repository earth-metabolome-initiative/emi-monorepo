//! Submodule defining traits to generate `quote` tokens for SQL constructs.

mod table;
pub use table::TableSynLike;
mod column;
pub use column::ColumnSynLike;
