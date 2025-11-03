//! Submodule providing the `VerticalSameAsColumnLike` trait for working
//! with columns that have vertical same-as relationships.

use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
};

use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike, TableLike};

use crate::traits::VerticalSameAsForeignKeyLike;
/// Trait for tables which may include vertical same-as relationships.
pub trait VerticalSameAsColumnLike: ColumnLike
where
    <Self::DB as DatabaseLike>::ForeignKey: VerticalSameAsForeignKeyLike<DB = Self::DB>,
{
    /// Returns an iterator over the vertical same-as foreign keys in the host
    /// table that reference this column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
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
    /// let child_table = db.table(None, "child").unwrap();
    /// let name_column = child_table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = child_table.column("age", &db).expect("Column 'age' should exist");
    /// assert_eq!(name_column.vertical_same_as_foreign_keys(&db).count(), 1);
    /// assert_eq!(age_column.vertical_same_as_foreign_keys(&db).count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn vertical_same_as_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey> {
        use crate::traits::same_as::vertical_same_as::vertical_same_as_table::VerticalSameAsTableLike;
        self.table(database).vertical_same_as_foreign_keys(database).filter(move |fk| {
            fk.host_columns(database).map(Borrow::borrow).any(|col: &Self| col == self)
        })
    }

    /// Returns the set of columns that are directly vertically same-as this
    /// column via vertical same-as foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
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
    /// let child_table = db.table(None, "child").unwrap();
    /// let name_column = child_table.column("name", &db).expect("Column 'name' should exist");
    /// let age_column = child_table.column("age", &db).expect("Column 'age' should exist");
    /// let parent_table = db.table(None, "parent").unwrap();
    /// let parent_name_column = parent_table.column("name", &db).expect("Column 'name' should exist");
    /// let parent_age_column = parent_table.column("age", &db).expect("Column 'age' should exist");
    /// assert_eq!(name_column.vertical_same_as_columns(&db), vec![parent_name_column]);
    /// assert_eq!(age_column.vertical_same_as_columns(&db), vec![parent_age_column]);
    /// # Ok(())
    /// # }
    /// ```
    fn vertical_same_as_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Vec<&'db <Self::DB as DatabaseLike>::Column> {
        self.vertical_same_as_foreign_keys(database)
            .map(|fk| fk.referenced_column_for_host_column(database, self.borrow()))
            .collect()
    }

    /// Returns the set of columns that are reachable from this column via
    /// vertical same-as foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    ///     CREATE TABLE grandparent (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    ///     CREATE TABLE parent (
    ///         id INT PRIMARY KEY REFERENCES grandparent(id),
    ///         parent_name TEXT,
    ///         FOREIGN KEY (id, parent_name) REFERENCES grandparent(id, name)
    ///     );
    ///     CREATE TABLE child (
    ///         id INT PRIMARY KEY REFERENCES parent(id),
    ///         child_name TEXT,
    ///         FOREIGN KEY (id, child_name) REFERENCES grandparent(id, name)
    ///     );
    ///   "#,
    /// )?;
    /// let child_table = db.table(None, "child").unwrap();
    /// let child_name_column =
    ///     child_table.column("child_name", &db).expect("Column 'child_name' should exist");
    /// let grandparent_table = db.table(None, "grandparent").unwrap();
    /// let grandparent_name_column =
    ///     grandparent_table.column("name", &db).expect("Column 'name' should exist");
    /// assert_eq!(
    ///     child_name_column.vertical_same_as_reachable_set(&db).into_iter().collect::<Vec<_>>(),
    ///     vec![grandparent_name_column]
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn vertical_same_as_reachable_set<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> HashSet<&'db <Self::DB as DatabaseLike>::Column> {
        let mut reachable_set = HashSet::new();
        for vertical_same_as_foreign_key in
            self.vertical_same_as_foreign_keys(database).map(Borrow::borrow)
        {
            let same_as_column = vertical_same_as_foreign_key
                .referenced_column_for_host_column(database, self.borrow());
            reachable_set.extend(same_as_column.vertical_same_as_reachable_set(database));
            reachable_set.insert(same_as_column);
        }
        reachable_set
    }

    /// Returns all columns that are vertically same-as this column, including
    /// those inferred via transitive relationships through ancestral tables.
    ///
    /// # Arguments
    ///
    /// * `database` - The database containing the tables and foreign keys.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    ///     CREATE TABLE grandparent (id INT PRIMARY KEY, name TEXT, UNIQUE(id, name));
    ///     CREATE TABLE parent (
    ///         id INT PRIMARY KEY REFERENCES grandparent(id),
    ///         parent_name TEXT,
    ///         FOREIGN KEY (id, parent_name) REFERENCES grandparent(id, name)
    ///     );
    ///     CREATE TABLE child (
    ///         id INT PRIMARY KEY REFERENCES parent(id),
    ///         child_name TEXT,
    ///         FOREIGN KEY (id, child_name) REFERENCES grandparent(id, name)
    ///     );
    ///   "#,
    /// )?;
    /// let child_table = db.table(None, "child").unwrap();
    /// let child_name_column =
    ///     child_table.column("child_name", &db).expect("Column 'child_name' should exist");
    /// let parent_table = db.table(None, "parent").unwrap();
    /// let parent_name_column =
    ///     parent_table.column("parent_name", &db).expect("Column 'parent_name' should exist");
    /// let grandparent_table = db.table(None, "grandparent").unwrap();
    /// let grandparent_name_column =
    ///     grandparent_table.column("name", &db).expect("Column 'name' should exist");
    /// assert_eq!(child_name_column.dominant_vertical_same_as_columns(&db), vec![parent_name_column]);
    /// # Ok(())
    /// # }
    /// ```
    fn dominant_vertical_same_as_columns<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut reachable_set: HashSet<&Self> =
            self.vertical_same_as_reachable_set(database).into_iter().map(Borrow::borrow).collect();
        // The frontier contains the set of columns which so far can only be reached
        // from the current column. Once a column in the frontier is found to be
        // reachable from another column in the reachable set, it is marked as true
        // and will not be used to expand the reachable set anymore.
        let mut frontier: HashMap<&Self, bool> = self
            .vertical_same_as_columns(database)
            .into_iter()
            .map(Borrow::borrow)
            .map(|c| (c, false))
            .collect();
        let table: &<Self::DB as DatabaseLike>::Table = self.table(database);
        let vertical_extension_tables: Vec<&<Self::DB as DatabaseLike>::Table> =
            table.ancestral_extended_tables(database);
        let mut changed = true;

        while changed {
            changed = false;
            for ancestor in vertical_extension_tables.iter() {
                for ancestor_column in ancestor.columns(database) {
                    let ancestor_column: &Self = ancestor_column.borrow();
                    // If the ancestor node is already in the reachable set, skip it.
                    if reachable_set.contains(ancestor_column) {
                        continue;
                    }

                    let ancestor_reachable_set: HashSet<&Self> = ancestor_column
                        .vertical_same_as_reachable_set(database)
                        .into_iter()
                        .map(Borrow::borrow)
                        .collect();

                    // We update the frontier to mark as true columns which we have now discovered
                    // can be reached also from this ancestor column.
                    for (frontier_column, is_reachable) in &mut frontier {
                        if !*is_reachable && ancestor_reachable_set.contains(frontier_column) {
                            *is_reachable = true;
                            changed = true;
                        }
                    }

                    // If the ancestor reachable set intersects with the current reachable
                    // set, then the ancestor column is inferred to be vertically same-as
                    // the current column. We then merge the ancestor reachable set into
                    // the current reachable set, and add the ancestor column to the
                    // frontier.
                    if !reachable_set.is_disjoint(&ancestor_reachable_set) {
                        reachable_set.insert(ancestor_column);
                        frontier.insert(ancestor_column, false);
                        reachable_set.extend(ancestor_reachable_set);
                        changed = true;
                    }
                }
            }
        }

        // We then consider as vertically same-as only those columns in the frontier
        // which are still marked as false, meaning they could not be reached
        // from any other column in the reachable set.
        let mut vertical_same_as_columns = frontier
            .into_iter()
            .filter_map(|(column, is_reachable)| if is_reachable { None } else { Some(column) })
            .collect::<Vec<_>>();
        vertical_same_as_columns.sort_unstable();

        vertical_same_as_columns
    }
}

impl<T> VerticalSameAsColumnLike for T
where
    T: ColumnLike,
    <T::DB as DatabaseLike>::ForeignKey: VerticalSameAsForeignKeyLike<DB = T::DB>,
{
}
