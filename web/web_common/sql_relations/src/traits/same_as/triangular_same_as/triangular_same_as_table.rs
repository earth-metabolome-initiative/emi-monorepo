//! Submodule providing the `TriangularSameAsTableLike` trait for working
//! with tables that have triangular same-as relationships.

use sql_traits::traits::{DatabaseLike, TableLike};

use crate::traits::TriangularSameAsForeignKeyLike;

/// Trait for tables which may include triangular same-as relationships.
pub trait TriangularSameAsTableLike: TableLike
where
    <Self::DB as DatabaseLike>::ForeignKey: TriangularSameAsForeignKeyLike<DB = Self::DB>,
{
    /// Returns an iterator over the foreign keys of this table that
    /// represent triangular same-as relationships.
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
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent (id INT PRIMARY KEY);
    /// CREATE TABLE parent (id INT PRIMARY KEY REFERENCES grandparent(id));
    /// CREATE TABLE grandparent_hyphotenuse (
    /// 	id INT PRIMARY KEY,
    /// 	grandparent_id INT REFERENCES grandparent(id),
    /// 	UNIQUE(id, grandparent_id)
    /// );
    /// CREATE TABLE child (
    /// 	id INT PRIMARY KEY REFERENCES parent(id),
    ///     triangular_id INT REFERENCES grandparent_hyphotenuse(id),
    ///     FOREIGN KEY (triangular_id, id) REFERENCES grandparent_hyphotenuse(id, grandparent_id)
    /// );
    /// "#,
    /// )?;
    /// let child = db.table(None, "child");
    /// let triangular_fks = child.triangular_same_as_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(triangular_fks.len(), 1, "Expected exactly one triangular same-as foreign key");
    /// # Ok(())
    /// # }
    /// ```
    fn triangular_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_triangular_same_as(database))
    }
}

impl<T> TriangularSameAsTableLike for T
where
    T: TableLike,
    <T::DB as DatabaseLike>::ForeignKey: TriangularSameAsForeignKeyLike<DB = T::DB>,
{
}
