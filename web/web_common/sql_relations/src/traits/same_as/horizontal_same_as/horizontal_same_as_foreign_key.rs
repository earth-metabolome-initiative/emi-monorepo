//! Submodule providing the `HorizontalSameAsForeignKeyLike` trait for
//! determining whether a foreign key relationship is a horizontal same-as
//! relationship.

use sql_traits::traits::{ColumnLike, DatabaseLike};

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
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent (id INT PRIMARY KEY, parent_name TEXT, UNIQUE(id, parent_name));
    /// CREATE TABLE brother (id INT PRIMARY KEY, brother_name TEXT, UNIQUE(id, brother_name));
    /// CREATE TABLE child (
    /// 	id INT PRIMARY KEY REFERENCES parent(id),
    ///     brother_id INT,
    /// 	child_name TEXT,
    ///     FOREIGN KEY (id, child_name) REFERENCES parent(id, parent_name),
    ///     FOREIGN KEY (brother_id, child_name) REFERENCES brother(id, brother_name)
    /// );
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child").unwrap();
    /// let [extension_primary_key, parent_fk, brother_fk] =
    ///     child_table.foreign_keys(&db).collect::<Vec<_>>()[..]
    /// else {
    ///     panic!("Expected exactly 3 foreign keys in child table");
    /// };
    ///
    /// assert!(extension_primary_key.is_extension_foreign_key(&db), "Expected extension primary key");
    /// assert!(
    ///     !parent_fk.is_horizontal_same_as(&db),
    ///     "Expected parent foreign key to not be horizontal same-as"
    /// );
    /// assert!(
    ///     brother_fk.is_horizontal_same_as(&db),
    ///     "Expected brother foreign key to be horizontal same-as"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn is_horizontal_same_as(&self, database: &Self::DB) -> bool {
        let Some(unique_index) = self.is_referenced_unique_key(database) else {
            return false;
        };

        unique_index.is_same_as(database) && !self.is_vertical_same_as(database)
    }

    /// Returns the tuple of host-referenced column pairs for this horizontal
    /// same-as foreign key, excluding the primary key columns.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent (id INT PRIMARY KEY, parent_name TEXT, UNIQUE(id, parent_name));
    /// CREATE TABLE brother (id INT PRIMARY KEY, brother_name TEXT, UNIQUE(id, brother_name));
    /// CREATE TABLE child (
    /// 	id INT PRIMARY KEY REFERENCES parent(id),
    /// 	child_name TEXT,
    ///     brother_id INT REFERENCES brother(id),
    ///     FOREIGN KEY (id, child_name) REFERENCES parent(id, parent_name),
    ///     FOREIGN KEY (brother_id, child_name) REFERENCES brother(id, brother_name)
    /// );
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent").unwrap();
    /// let brother_table = db.table(None, "brother").unwrap();
    /// let brother_name = brother_table
    ///     .column("brother_name", &db)
    ///     .expect("Column 'brother_name' should exist in brother table");
    /// let child_table = db.table(None, "child").unwrap();
    /// let child_name = child_table
    ///     .column("child_name", &db)
    ///     .expect("Column 'child_name' should exist in child table");
    /// let foreign_keys = child_table.foreign_keys(&db).collect::<Vec<_>>();
    /// let [pk_fk, vertical_fk, horizontal_fk] = &foreign_keys.as_slice() else {
    ///     panic!("Expected exactly 3 foreign keys in child table");
    /// };
    /// assert!(pk_fk.is_extension_foreign_key(&db), "Expected extension primary key");
    /// assert!(vertical_fk.is_vertical_same_as(&db), "Expected vertical same-as foreign key");
    /// assert!(!vertical_fk.is_horizontal_same_as(&db), "Expected non-horizontal same-as foreign key");
    /// assert!(horizontal_fk.is_horizontal_same_as(&db), "Expected horizontal same-as foreign key");
    /// assert!(!horizontal_fk.is_vertical_same_as(&db), "Expected non-vertical same-as foreign key");
    ///
    /// assert_eq!(
    ///     horizontal_fk.horizontal_same_as_column_pair(&db),
    ///     Some((child_name, brother_name)),
    ///     "Expected horizontal same-as column pair to be (brother.name, child.name)"
    /// );
    ///
    /// # Ok(())
    /// # }
    /// ```
    fn horizontal_same_as_column_pair<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Option<(&'db <Self::DB as DatabaseLike>::Column, &'db <Self::DB as DatabaseLike>::Column)>
    {
        if !self.is_horizontal_same_as(database) {
            return None;
        }
        self.host_columns(database)
            .zip(self.referenced_columns(database))
            .find(|(_, referenced_column)| !referenced_column.is_primary_key(database))
    }
}

impl<T> HorizontalSameAsForeignKeyLike for T where T: VerticalSameAsForeignKeyLike {}
