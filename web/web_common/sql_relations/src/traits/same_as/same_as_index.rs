//! Submodule defining the `SameAsIndex` trait which characterizes whether an
//! index can be used to define a same-as relationship.

use sql_traits::traits::{TableLike, UniqueIndexLike};

/// Trait characterizing whether an index can be used to define a same-as
/// relationship, i.e. it is a unique index over a single column.
pub trait SameAsIndexLike: UniqueIndexLike {
    /// Returns whether the index can be used to define a same-as relationship.
    ///
    /// # Arguments
    ///
    /// * `database` - The database context in which the index exists.
    /// * `table` - The table to which the index belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE with_same_as (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    /// CREATE TABLE no_same_as_one (id INT PRIMARY KEY, name TEXT, UNIQUE(name));
    /// CREATE TABLE no_same_as_two (id INT PRIMARY KEY);
    /// "#,
    /// )?;
    ///
    /// let table_with_same_as = db.table(None, "with_same_as");
    /// let same_as_indices = table_with_same_as.same_as_indices(&db).collect::<Vec<_>>();
    /// assert_eq!(same_as_indices.len(), 1, "Expected exactly one same-as index");
    ///
    /// let table_no_same_as_one = db.table(None, "no_same_as_one");
    /// assert_eq!(table_no_same_as_one.same_as_indices(&db).count(), 0, "Expected no same-as indices");
    /// let table_no_same_as_two = db.table(None, "no_same_as_two");
    /// assert_eq!(table_no_same_as_two.same_as_indices(&db).count(), 0, "Expected no same-as indices");
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn is_same_as(&self, database: &Self::DB) -> bool {
        // Next, we retrieve the columns associated with the index.
        let columns = self.columns(database).collect::<Vec<_>>();

        // We expect that all of the columns in the primary key of the table are also in
        // the index.
        let primary_key_columns =
            self.table(database).primary_key_columns(database).collect::<Vec<_>>();

        // If any of the primary key columns are not in the index, it cannot be a
        // same-as index
        primary_key_columns.iter().all(|pk_col| columns.contains(pk_col))
    }
}

impl<T> SameAsIndexLike for T where T: UniqueIndexLike {}
