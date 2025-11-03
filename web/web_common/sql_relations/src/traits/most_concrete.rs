//! Submodule defining the `MostConcrete` trait which provides method relative
//! to dispatching the most-concrete table in an inheritance hierarchy.

use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

/// Trait describing a database characterized by table inheritance.
pub trait InheritableDatabaseLike: DatabaseLike {
    /// Returns the name of the column which indicates the most concrete
    /// table in an inheritance hierarchy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY)")?;
    /// assert_eq!(db.most_concrete_table_column_name(), "most_concrete_table");
    /// # Ok(())
    /// # }
    /// ```
    fn most_concrete_table_column_name(&self) -> &str;
}

/// Trait characterizing whether a table has the `most_concrete_table` column,
/// i.e. a column indicating the most concrete table in an inheritance
/// hierarchy.
pub trait MostConcreteTableLike: TableLike
where
    Self::DB: InheritableDatabaseLike,
{
    /// Returns the column which indicates the most concrete table in an
    /// inheritance hierarchy, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - The database the table belongs to.
    /// * `recursive` - Whether to search ancestor tables for a
    ///   `most_concrete_table` column.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE parent_table (id INT PRIMARY KEY, most_concrete_table TEXT);
    ///   CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    ///   CREATE TABLE grandchild_table (id INT PRIMARY KEY REFERENCES child_table(id));
    ///   CREATE TABLE unrelated_table (id INT PRIMARY KEY);
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let grandchild_table = db.table(None, "grandchild_table").unwrap();
    /// let unrelated_table = db.table(None, "unrelated_table").unwrap();
    /// let most_concrete_column = parent_table.most_concrete_column(&db, false).unwrap();
    /// assert_eq!(most_concrete_column.column_name(), "most_concrete_table");
    /// let child_most_concrete_column = child_table.most_concrete_column(&db, true).unwrap();
    /// assert_eq!(child_most_concrete_column.column_name(), "most_concrete_table");
    /// let grandchild_most_concrete_column = grandchild_table.most_concrete_column(&db, true).unwrap();
    /// assert_eq!(grandchild_most_concrete_column.column_name(), "most_concrete_table");
    /// let unrelated_column = unrelated_table.column("id", &db).unwrap();
    /// assert!(unrelated_column.column_name() != "most_concrete_table");
    /// # Ok(())
    /// # }
    /// ```
    fn most_concrete_column<'db>(
        &'db self,
        database: &'db Self::DB,
        recursive: bool,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        if let Some(column) = self.column(database.most_concrete_table_column_name(), database) {
            return Some(column);
        };
        if recursive {
            for parent in self.extended_tables(database) {
                if let Some(col) = parent.most_concrete_column(database, true) {
                    return Some(col);
                }
            }
        }
        None
    }
}

impl<T> MostConcreteTableLike for T
where
    T: TableLike,
    T::DB: InheritableDatabaseLike,
{
}

/// Trait characterizing whether a column is the most concrete table column.

pub trait MostConcreteColumnLike: ColumnLike
where
    Self::DB: InheritableDatabaseLike,
{
    /// Returns `true` if the column is the most concrete table column.
    ///
    /// # Arguments
    ///
    /// * `database` - The database the column belongs to.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_relations::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE parent_table (id INT PRIMARY KEY, most_concrete_table TEXT);
    ///   CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    ///   CREATE TABLE grandchild_table (id INT PRIMARY KEY REFERENCES child_table(id));
    ///   CREATE TABLE unrelated_table (id INT PRIMARY KEY);
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let grandchild_table = db.table(None, "grandchild_table").unwrap();
    /// let unrelated_table = db.table(None, "unrelated_table").unwrap();
    /// let most_concrete_column = parent_table.most_concrete_column(&db, false).unwrap();
    /// assert!(most_concrete_column.is_most_concrete(&db));
    /// let child_most_concrete_column = child_table.most_concrete_column(&db, true).unwrap();
    /// assert!(child_most_concrete_column.is_most_concrete(&db));
    /// let grandchild_most_concrete_column = grandchild_table.most_concrete_column(&db, true).unwrap();
    /// assert!(grandchild_most_concrete_column.is_most_concrete(&db));
    /// let unrelated_column = unrelated_table.column("id", &db).unwrap();
    /// assert!(!unrelated_column.is_most_concrete(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_most_concrete(&self, database: &Self::DB) -> bool {
        self.column_name() == database.most_concrete_table_column_name()
    }
}

impl<T> MostConcreteColumnLike for T
where
    T: ColumnLike,
    T::DB: InheritableDatabaseLike,
{
}
