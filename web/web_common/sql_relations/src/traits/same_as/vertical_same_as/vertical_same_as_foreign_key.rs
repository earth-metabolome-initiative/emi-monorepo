//! Submodule providing the `VerticalSameAsForeignKeyLike` trait for
//! determining whether a foreign key relationship is an vertical same-as
//! relationship.

use sql_traits::traits::{DatabaseLike, ForeignKeyLike, TableLike};

use crate::traits::same_as::same_as_index::SameAsIndexLike;

/// Trait for foreign keys that can be checked for being vertical same-as
/// relationships.
pub trait VerticalSameAsForeignKeyLike: ForeignKeyLike
where
    <Self::DB as DatabaseLike>::UniqueIndex: SameAsIndexLike<DB = Self::DB>,
{
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
    /// let [extension_primary_key, parent_fk, brother_fk] =
    ///     child_table.foreign_keys(&db).collect::<Vec<_>>()[..]
    /// else {
    ///     panic!("Expected exactly 3 foreign keys in child table");
    /// };
    /// assert!(extension_primary_key.is_extension_foreign_key(&db), "Expected extension primary key");
    /// assert!(
    ///     parent_fk.is_vertical_same_as(&db),
    ///     "Expected parent foreign key to be vertical same-as"
    /// );
    /// assert!(
    ///     !brother_fk.is_vertical_same_as(&db),
    ///     "Expected brother foreign key to not be vertical same-as"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_vertical_same_as(&self, database: &Self::DB) -> bool {
        if !self.includes_host_primary_key(database) {
            return false;
        }

        let host_table = self.host_table(database);
        let referenced_table = self.referenced_table(database);

        if !host_table.is_descendant_of(database, &referenced_table) {
            return false;
        }

        let Some(unique_index) = self.is_referenced_unique_key(database) else {
            return false;
        };

        unique_index.is_same_as(database)
    }
}

impl<T> VerticalSameAsForeignKeyLike for T
where
    T: ForeignKeyLike,
    <T::DB as DatabaseLike>::UniqueIndex: SameAsIndexLike<DB = T::DB>,
{
}
