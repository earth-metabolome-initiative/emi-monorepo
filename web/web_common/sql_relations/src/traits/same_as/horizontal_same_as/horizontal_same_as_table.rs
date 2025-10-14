//! Submodule providing the `HorizontalSameAsTableLike` trait for working
//! with tables that have horizontal same-as relationships.

use sql_traits::traits::TableLike;

use crate::traits::HorizontalSameAsForeignKeyLike;

/// Trait for tables which may include horizontal same-as relationships.
pub trait HorizontalSameAsTableLike:
    TableLike<ForeignKey = <Self as HorizontalSameAsTableLike>::HorizontalSameAsForeignKey>
{
    /// The type of the foreign keys in this table that may be horizontal
    /// same-as relationships.
    type HorizontalSameAsForeignKey: HorizontalSameAsForeignKeyLike<Database = Self::Database, Table = Self>;

    /// Returns an iterator over the foreign keys of this table that
    /// represent horizontal same-as relationships.
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
    /// let horizontal_same_as_fks =
    ///     child_table.horizontal_same_as_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(
    ///     horizontal_same_as_fks.len(),
    ///     1,
    ///     "Expected exactly one horizontal same-as foreign key"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn horizontal_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
    ) -> impl Iterator<Item = &'db Self::HorizontalSameAsForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_horizontal_same_as(database))
    }
}

impl<T> HorizontalSameAsTableLike for T
where
    T: TableLike,
    T::ForeignKey: HorizontalSameAsForeignKeyLike,
{
    type HorizontalSameAsForeignKey = T::ForeignKey;
}
