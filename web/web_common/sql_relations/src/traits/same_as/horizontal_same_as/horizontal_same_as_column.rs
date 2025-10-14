//! Submodule providing the `HorizontalSameAsColumnLike` trait for working
//! with columns that have horizontal same-as relationships.

use std::collections::HashSet;

use sql_traits::traits::{ColumnLike, ForeignKeyLike};

use crate::traits::HorizontalSameAsForeignKeyLike;
/// Trait for tables which may include horizontal same-as relationships.
pub trait HorizontalSameAsColumnLike:
    ColumnLike<ForeignKey = <Self as HorizontalSameAsColumnLike>::HorizontalSameAsForeignKey>
{
    /// The type of the foreign keys in this table that may be horizontal
    /// same-as relationships.
    type HorizontalSameAsForeignKey: HorizontalSameAsForeignKeyLike<
            Database = Self::Database,
            Table = Self::Table,
            Column = Self,
        >;

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
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> impl Iterator<Item = &'db Self::HorizontalSameAsForeignKey> {
        use crate::traits::same_as::HorizontalSameAsTableLike;
        HorizontalSameAsTableLike::horizontal_same_as_foreign_keys(host_table, database)
            .filter(move |fk| fk.host_columns(database).any(|col| col == self))
    }

    /// Returns the set of columns that are reachable from this column via
    /// horizontal same-as foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    /// * `host_table` - The table that may contain horizontal same-as foreign
    ///  keys referencing this column whose reachable set is being computed.
    fn horizontal_same_as_reachable_set<'db>(
        &'db self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> HashSet<&'db Self> {
        let mut reachable_set = HashSet::new();
        for horizontal_same_as_foreign_key in
            self.horizontal_same_as_foreign_keys(database, host_table)
        {
            let same_as_column = horizontal_same_as_foreign_key
                .referenced_column_for_host_column(database, self);
            let referenced_table = horizontal_same_as_foreign_key.referenced_table(database);
            reachable_set.extend(
                same_as_column.horizontal_same_as_reachable_set(database, referenced_table),
            );
            reachable_set.insert(same_as_column);
        }
        reachable_set
    }
}

impl<T> HorizontalSameAsColumnLike for T
where
    T: ColumnLike,
    T::ForeignKey:
        HorizontalSameAsForeignKeyLike<Database = T::Database, Table = T::Table, Column = T>,
{
    type HorizontalSameAsForeignKey = T::ForeignKey;
}
