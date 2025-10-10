//! Submodule defining the `SameAsTable` trait which provides methods for
//! tables which include same-as relationships.

use sql_traits::traits::TableLike;

use crate::traits::SameAsIndexLike;

/// Trait characterizing whether an index can be used to define a same-as
/// relationship, i.e. it is a unique index over a single column.
pub trait SameAsTableLike: TableLike<UniqueIndex = <Self as SameAsTableLike>::SameAsIndex> {
    /// The type of index which can be used to define a same-as relationship.
    type SameAsIndex: SameAsIndexLike<Database = Self::Database, Table = Self>;

    /// Returns the indices on the table which can be used to define same-as
    /// relationships.
    ///
    /// # Arguments
    ///
    /// * `database` - The database context in which the table exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    /// CREATE TABLE with_same_as (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    /// CREATE TABLE no_same_as_one (id INT PRIMARY KEY, name TEXT, UNIQUE(name));
    /// CREATE TABLE no_same_as_two (id INT PRIMARY KEY);
    /// "#,
    /// )?;
    /// let table_with_same_as = db.table(None, "with_same_as");
    /// let same_as_indices = table_with_same_as.same_as_indices(&db).collect::<Vec<_>>();
    /// assert_eq!(same_as_indices.len(), 1, "Expected exactly one same-as index");
    /// let table_no_same_as_one = db.table(None, "no_same_as_one");
    /// assert_eq!(table_no_same_as_one.same_as_indices(&db).count(), 0, "Expected no same-as indices");
    /// let table_no_same_as_two = db.table(None, "no_same_as_two");
    /// assert_eq!(table_no_same_as_two.same_as_indices(&db).count(), 0, "Expected no same-as indices");
    /// # Ok(())
    /// # }
    /// ```
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
