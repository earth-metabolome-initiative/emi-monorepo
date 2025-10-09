//! Submodule defining the `SameAsTable` trait which provides methods for
//! tables which include same-as relationships.

use sql_traits::traits::TableLike;

use crate::traits::SameAsIndexLike;

/// Trait characterizing whether an index can be used to define a same-as
/// relationship, i.e. it is a unique index over a single column.
pub trait SameAsTableLike: TableLike<UniqueIndex = <Self as SameAsTableLike>::SameAsIndex> {
    /// The type of index which can be used to define a same-as relationship.
    type SameAsIndex: SameAsIndexLike<Database = Self::Database, Table = Self>;

    /// Returns the indices on the table which can be used to define
    fn same_as_indices<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::SameAsIndex>
    where
        Self: 'db,
    {
        self.unique_indices(database).filter(|index| index.is_same_as(database, self))
    }
}

impl<T> SameAsTableLike for T
where
    T: TableLike,
    T::UniqueIndex: SameAsIndexLike,
{
    type SameAsIndex = T::UniqueIndex;
}
