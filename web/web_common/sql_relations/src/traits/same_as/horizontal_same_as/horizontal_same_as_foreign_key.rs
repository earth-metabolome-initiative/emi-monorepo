//! Submodule providing the `HorizontalSameAsForeignKeyLike` trait for
//! determining whether a foreign key relationship is a horizontal same-as
//! relationship.

use crate::traits::{SameAsIndexLike, VerticalSameAsForeignKeyLike};

/// Trait for foreign keys that can be checked for being horizontal same-as
/// relationships.
pub trait HorizontalSameAsForeignKeyLike: VerticalSameAsForeignKeyLike {
    /// Returns whether this key column usage is an horizontal same-as
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
    /// let [extension_primary_key, parent_fk, brother_fk] =
    ///     child_table.foreign_keys(&db).collect::<Vec<_>>()[..]
    /// else {
    ///     panic!("Expected exactly 3 foreign keys in child table");
    /// };
    ///
    /// assert!(
    ///     extension_primary_key.is_extension_foreign_key(&db, child_table),
    ///     "Expected extension primary key"
    /// );
    /// assert!(
    ///     !parent_fk.is_horizontal_same_as(&db, child_table),
    ///     "Expected parent foreign key to not be horizontal same-as"
    /// );
    /// assert!(
    ///     brother_fk.is_horizontal_same_as(&db, child_table),
    ///     "Expected brother foreign key to be horizontal same-as"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_horizontal_same_as(&self, database: &Self::Database, host_table: &Self::Table) -> bool {
        let referenced_table = self.referenced_table(database);

        let Some(unique_index) = self.is_referenced_unique_key(database) else {
            return false;
        };

        unique_index.is_same_as(database, &referenced_table)
            && !self.is_vertical_same_as(database, host_table)
    }
}

impl<T> HorizontalSameAsForeignKeyLike for T where T: VerticalSameAsForeignKeyLike {}
