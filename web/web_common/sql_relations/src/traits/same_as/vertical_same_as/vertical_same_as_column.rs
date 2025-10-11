//! Submodule providing the `VerticalSameAsColumnLike` trait for working
//! with columns that have vertical same-as relationships.

use std::collections::HashSet;

use sql_traits::traits::{ColumnLike, ForeignKeyLike};

use crate::traits::VerticalSameAsForeignKeyLike;
/// Trait for tables which may include vertical same-as relationships.
pub trait VerticalSameAsColumnLike:
    ColumnLike<ForeignKey = <Self as VerticalSameAsColumnLike>::VerticalSameAsForeignKey>
{
    /// The type of the foreign keys in this table that may be vertical same-as
    /// relationships.
    type VerticalSameAsForeignKey: VerticalSameAsForeignKeyLike<Database = Self::Database, Table = Self::Table, Column = Self>;

    /// Returns an iterator over the vertical same-as foreign keys in the host
    /// table that reference this column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    /// * `host_table` - The table that may contain vertical same-as foreign
    ///   keys referencing this column.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = SqlParserDatabase::from_sql(
    ///     r#"
    ///     CREATE TABLE parent (id INT PRIMARY KEY, name TEXT, age SMALLINT, UNIQUE(id, name), UNIQUE(id, age));
    ///     CREATE TABLE child (
    ///         id INT PRIMARY KEY REFERENCES parent(id),
    ///         name TEXT,
    ///         age SMALLINT,
    ///         FOREIGN KEY (id, name) REFERENCES parent(id, name),
    ///         FOREIGN KEY (id, age) REFERENCES parent(id, age)
    ///     );
    ///   "#,
    /// )?;
    /// let child_table = db.table(None, "child");
    /// let name_column = child_table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = child_table.column("age", &db).expect("Column 'age' should exist");
    /// assert_eq!(name_column.vertical_same_as_foreign_keys(&db, child_table).count(), 1);
    /// assert_eq!(age_column.vertical_same_as_foreign_keys(&db, child_table).count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn vertical_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> impl Iterator<Item = &'db Self::VerticalSameAsForeignKey> {
        use crate::traits::same_as::vertical_same_as::vertical_same_as_table::VerticalSameAsTableLike;
        host_table
            .vertical_same_as_foreign_keys(database)
            .filter(move |fk| fk.host_columns(database, host_table).any(|col| col == self))
    }

    /// Returns the set of columns that are reachable from this column via
    /// vertical same-as foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    /// * `host_table` - The table that may contain vertical same-as foreign
    ///  keys referencing this column whose reachable set is being computed.
    fn vertical_same_as_reachable_set<'db>(
        &'db self,
        database: &'db Self::Database,
        host_table: &'db Self::Table,
    ) -> HashSet<&'db Self> {
        let mut reachable_set = HashSet::new();
        for vertical_same_as_foreign_key in self.vertical_same_as_foreign_keys(database, host_table)
        {
            let same_as_column = vertical_same_as_foreign_key
                .referenced_column_for_host_column(database, host_table, self);
            let referenced_table = vertical_same_as_foreign_key.referenced_table(database);
            reachable_set
                .extend(same_as_column.vertical_same_as_reachable_set(database, referenced_table));
            reachable_set.insert(same_as_column);
        }
        reachable_set
    }
}

impl<T> VerticalSameAsColumnLike for T
where
    T: ColumnLike,
    T::ForeignKey:
        VerticalSameAsForeignKeyLike<Database = T::Database, Table = T::Table, Column = T>,
{
    type VerticalSameAsForeignKey = T::ForeignKey;
}
