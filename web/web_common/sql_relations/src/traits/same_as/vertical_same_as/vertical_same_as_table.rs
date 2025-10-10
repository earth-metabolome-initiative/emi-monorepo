//! Submodule providing the `VerticalSameAsTableLike` trait for working
//! with tables that have vertical same-as relationships.

use sql_traits::traits::TableLike;

use crate::traits::VerticalSameAsForeignKeyLike;

/// Trait for tables which may include vertical same-as relationships.
pub trait VerticalSameAsTableLike:
    TableLike<ForeignKey = <Self as VerticalSameAsTableLike>::VerticalSameAsForeignKey>
{
    /// The type of the foreign keys in this table that may be vertical same-as
    /// relationships.
    type VerticalSameAsForeignKey: VerticalSameAsForeignKeyLike<Database = Self::Database, Table = Self>;

    /// Returns an iterator over the foreign keys of this table that
    /// represent vertical same-as relationships.
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
    /// CREATE TABLE parent (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    /// CREATE TABLE brother (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    /// CREATE TABLE child (
    /// 	id INT PRIMARY KEY REFERENCES parent(id),
    ///     brother_id INT,
    /// 	name TEXT,
    ///     FOREIGN KEY (id, name) REFERENCES parent(id, name),
    ///     FOREIGN KEY (brother_id, name) REFERENCES brother(id, name)
    /// );
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child");
    /// let vertical_same_as_fks = child_table.vertical_same_as_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(vertical_same_as_fks.len(), 1, "Expected exactly one vertical same-as foreign key");
    /// # Ok(())
    /// # }
    /// ```
    fn vertical_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::VerticalSameAsForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_vertical_same_as(database, self))
    }
}

impl<T> VerticalSameAsTableLike for T
where
    T: TableLike,
    T::ForeignKey: VerticalSameAsForeignKeyLike,
{
    type VerticalSameAsForeignKey = T::ForeignKey;
}
