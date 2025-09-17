//! Submodule defining traits used in code generation.

mod column_like;
mod table_like;

pub use column_like::ColumnLike;
pub use table_like::TableLike;
