//! Submodule providing the `TriangularSameAsColumnLike` trait for
//! querying triangular same-as relationships on foreign keys associated
//! to a column.

use sql_traits::traits::{ColumnLike, DatabaseLike};

use crate::traits::TriangularSameAsForeignKeyLike;

/// Trait for columns potentially involved in triangular same-as
/// relationships via foreign keys.
pub trait TriangularSameAsColumnLike: ColumnLike {
    /// Returns whether the column is the key of a triangular same-as
    /// relationship via foreign keys.
    fn has_triangular_same_as_foreign_key(&self, database: &Self::DB) -> bool {
        self.triangular_same_as_foreign_keys(database).next().is_some()
    }

    /// Returns the triangular same-as foreign keys defined on the column.
    fn triangular_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey> {
        self.foreign_keys(database).filter(move |fk| fk.is_triangular_same_as(database))
    }
}

impl<T> TriangularSameAsColumnLike for T where T: ColumnLike {}
