//! Submodule providing traits for describing SQL-like entities.

pub mod column;
pub mod database;
pub mod table;
pub use column::ColumnLike;
pub use database::DatabaseLike;
pub use table::TableLike;
pub mod check_constraint;
pub use check_constraint::CheckConstraintLike;
pub mod unique_index;
pub use unique_index::UniqueIndexLike;
pub mod foreign_key;
pub use foreign_key::ForeignKeyLike;

/// Trait for associating a metadata struct to a given type.
pub trait Metadata {
    /// The associated metadata type.
    type Meta;
}

