//! Submodule providing the `VerticalSameAsForeignKeyLike` trait for
//! determining whether a foreign key relationship is an vertical same-as
//! relationship.

use sql_traits::traits::{ForeignKeyLike, TableLike};

use crate::traits::same_as::same_as_index::SameAsIndexLike;

/// Trait for foreign keys that can be checked for being vertical same-as
/// relationships.
pub trait VerticalSameAsForeignKeyLike:
    ForeignKeyLike<UniqueIndex = <Self as VerticalSameAsForeignKeyLike>::SameAsIndex>
{
    /// The type of the same-as unique index, if this foreign key is an
    /// vertical same-as relationship.
    type SameAsIndex: SameAsIndexLike<Database = Self::Database, Table = Self::Table>;

    /// Returns whether this key column usage is an vertical same-as
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables.
    /// * `host_table` - The table containing the foreign key.
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
    /// let mut foreign_keys = child_table.foreign_keys(&db);
    /// let parent_fk = foreign_keys.next().expect("Expected parent foreign key");
    /// let brother_fk = foreign_keys.next().expect("Expected brother foreign key");
    /// assert!(
    ///     parent_fk.is_vertical_same_as(&db, child_table),
    ///     "Expected parent foreign key to be vertical same-as"
    /// );
    /// assert!(
    ///     !brother_fk.is_vertical_same_as(&db, child_table),
    ///     "Expected brother foreign key to not be vertical same-as"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_vertical_same_as(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        if !self.includes_host_primary_key(database, host_table) {
            return false;
        }

        let referenced_table = self.referenced_table(database);

        if !host_table.is_descendant_of(database, &referenced_table) {
            return false;
        }

        let Some(unique_index) = self.is_referenced_unique_key(database) else {
            return false;
        };

        unique_index.is_same_as(database, &referenced_table)
    }
}

impl<T> VerticalSameAsForeignKeyLike for T
where
    T: ForeignKeyLike,
    T::UniqueIndex: SameAsIndexLike,
{
    type SameAsIndex = T::UniqueIndex;
}
