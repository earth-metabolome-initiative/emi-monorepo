//! Submodule providing the `HorizontalSameAsColumnLike` trait for working
//! with columns that have horizontal same-as relationships.

use std::borrow::Borrow;

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike};

use crate::traits::HorizontalSameAsForeignKeyLike;
/// Trait for tables which may include horizontal same-as relationships.
pub trait HorizontalSameAsColumnLike: ColumnLike
where
    <Self::DB as DatabaseLike>::ForeignKey: HorizontalSameAsForeignKeyLike<DB = Self::DB>,
{
    /// Returns an iterator over the horizontal same-as foreign keys in the host
    /// table that reference this column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    /// * `host_table` - The table that may contain horizontal same-as foreign
    ///   keys referencing this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE right (id SERIAL PRIMARY KEY, name TEXT, age INT, UNIQUE(id, name), UNIQUE(id, age));
    /// CREATE TABLE left (
    ///     id SERIAL PRIMARY KEY,
    ///     right_id INTEGER REFERENCES right(id),
    ///     name TEXT,
    ///     age INT,
    ///     FOREIGN KEY (right_id, name) REFERENCES right(id, name),
    ///     FOREIGN KEY (right_id, age) REFERENCES right(id, age)
    /// );
    ///   "#,
    /// )?;
    /// let left_table = db.table(None, "left");
    /// let name_column = left_table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = left_table.column("age", &db).expect("Column 'age' should exist");
    /// assert_eq!(name_column.horizontal_same_as_foreign_keys(&db, left_table).count(), 1);
    /// assert_eq!(age_column.horizontal_same_as_foreign_keys(&db, left_table).count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn horizontal_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
        host_table: &'db <Self::DB as DatabaseLike>::Table,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey> {
        use crate::traits::same_as::HorizontalSameAsTableLike;
        HorizontalSameAsTableLike::horizontal_same_as_foreign_keys(host_table, database).filter(
            move |fk| fk.host_columns(database).map(Borrow::borrow).any(|col: &Self| col == self),
        )
    }
}

impl<T> HorizontalSameAsColumnLike for T
where
    T: ColumnLike,
    <T::DB as DatabaseLike>::ForeignKey: HorizontalSameAsForeignKeyLike<DB = T::DB>,
{
}
