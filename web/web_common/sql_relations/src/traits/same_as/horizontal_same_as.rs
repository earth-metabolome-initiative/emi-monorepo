//! Submodule defining whether a relationship defined by a foreign key is an
//! horizontal same-as relationship, i.e. a same-as relationship between a table
//! and another table which is not its ancestor.

mod horizontal_same_as_foreign_key;
pub use horizontal_same_as_foreign_key::HorizontalSameAsForeignKeyLike;
mod horizontal_same_as_table;
pub use horizontal_same_as_table::HorizontalSameAsTableLike;
