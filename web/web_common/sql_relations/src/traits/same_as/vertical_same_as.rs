//! Submodule defining whether a relationship defined by a foreign key is an
//! vertical same-as relationship, i.e. a same-as relationship between a table
//! and one of its ancestors.

mod vertical_same_as_foreign_key;
pub use vertical_same_as_foreign_key::VerticalSameAsForeignKeyLike;
mod vertical_same_as_table;
pub use vertical_same_as_table::VerticalSameAsTableLike;
mod vertical_same_as_column;
pub use vertical_same_as_column::VerticalSameAsColumnLike;
