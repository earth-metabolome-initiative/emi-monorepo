//! Submodule providing the `TriangularSameAsTableLike` trait for working
//! with tables that have triangular same-as relationships.

use sql_traits::traits::TableLike;

use crate::traits::TriangularSameAsForeignKeyLike;

/// Trait for tables which may include triangular same-as relationships.
pub trait TriangularSameAsTableLike:
    TableLike<ForeignKey = <Self as TriangularSameAsTableLike>::TriangularSameAsForeignKey>
{
    /// The type of the foreign keys in this table that may be triangular
    /// same-as relationships.
    type TriangularSameAsForeignKey: TriangularSameAsForeignKeyLike<Database = Self::Database, Table = Self>;

    /// Returns an iterator over the foreign keys of this table that
    /// represent triangular same-as relationships.
    ///
    /// # Arguments
    ///
    /// * `database` - The database context in which the table exists.
    fn triangular_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::TriangularSameAsForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_triangular_same_as(database))
    }
}

impl<T> TriangularSameAsTableLike for T
where
    T: TableLike,
    T::ForeignKey: TriangularSameAsForeignKeyLike,
{
    type TriangularSameAsForeignKey = T::ForeignKey;
}
