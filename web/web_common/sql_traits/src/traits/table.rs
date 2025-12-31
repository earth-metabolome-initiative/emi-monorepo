//! Submodule providing a trait for describing SQL Table-like entities.

use std::{borrow::Borrow, fmt::Debug, hash::Hash};

use crate::traits::{
    ColumnLike, DatabaseLike, ForeignKeyLike, Metadata, check_constraint::CheckConstraintLike,
};

/// A trait for types that can be treated as SQL tables.
pub trait TableLike:
    Debug
    + Clone
    + Hash
    + Ord
    + Eq
    + Metadata
    + Borrow<<<Self as TableLike>::DB as DatabaseLike>::Table>
{
    /// The database type the table belongs to.
    type DB: DatabaseLike<Table: Borrow<Self>>;

    /// Returns the name of the table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE mytable (id INT);")?;
    /// let table = db.table(None, "mytable").unwrap();
    /// assert_eq!(table.table_name(), "mytable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_name(&self) -> &str;

    /// Returns whether the table has a snake_case name.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT);
    /// CREATE TABLE MyTable (id INT);
    /// "#,
    /// )?;
    /// let snake_case_table = db.table(None, "my_table").unwrap();
    /// assert!(snake_case_table.is_snake_case());
    /// let non_snake_case_table = db.table(None, "MyTable").unwrap();
    /// assert!(!non_snake_case_table.is_snake_case());
    /// # Ok(())
    /// # }
    /// ```
    fn is_snake_case(&self) -> bool {
        let name = self.table_name();
        name.chars().all(|c| c.is_lowercase() || c == '_')
    }

    /// Returns the documentation of the table, if any.
    fn table_doc<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db;

    /// The schema name of the table, if it has one.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"CREATE TABLE my_schema.my_table_with_schema (id INT);
    /// CREATE TABLE my_table (id INT);"#,
    /// )?;
    /// let table_no_schema = db.table(None, "my_table").unwrap();
    /// assert_eq!(table_no_schema.table_schema(), None);
    /// let table_with_schema = db.table(Some("my_schema"), "my_table_with_schema").unwrap();
    /// assert_eq!(table_with_schema.table_schema(), Some("my_schema"));
    /// # Ok(())
    /// # }
    /// ```
    fn table_schema(&self) -> Option<&str>;

    /// Iterates over the columns of the table using the provided schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let column_names: Vec<&str> = table.columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(column_names, vec!["id", "name"]);
    /// # Ok(())
    /// # }
    /// ```
    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db;

    /// Returns whether any of the columns of the table are generated.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_other_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    ///
    /// let table = db.table(None, "my_table").unwrap();
    /// assert!(table.has_generated_columns(&db));
    /// let other_table = db.table(None, "my_other_table").unwrap();
    /// assert!(!other_table.has_generated_columns(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_generated_columns(&self, database: &Self::DB) -> bool {
        self.columns(database).any(ColumnLike::is_generated)
    }

    /// Returns the number of columns in the table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT, age INT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.number_of_columns(&db), 3);
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn number_of_columns(&self, database: &Self::DB) -> usize {
        self.columns(database).count()
    }

    /// Returns the corresponding column by name, if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the column to retrieve.
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT, name TEXT);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// assert_eq!(id_column.column_name(), "id");
    /// let non_existent_column = table.column("non_existent", &db);
    /// assert!(non_existent_column.is_none());
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn column<'db>(
        &'db self,
        name: &str,
        database: &'db Self::DB,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        TableLike::columns(self, database).find(|col| col.column_name() == name)
    }

    /// Returns whether the provided column belongs to this table.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to check.
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE table1 (id INT, name TEXT);
    /// CREATE TABLE table2 (id INT, description TEXT);
    /// "#,
    /// )?;
    /// let table1 = db.table(None, "table1").unwrap();
    /// let table2 = db.table(None, "table2").unwrap();
    /// let table1_id = table1.column("id", &db).expect("Column 'id' should exist in table1");
    /// let table2_id = table2.column("id", &db).expect("Column 'id' should exist in table2");
    /// assert!(table1.has_column(table1_id, &db));
    /// assert!(!table1.has_column(table2_id, &db));
    /// assert!(table2.has_column(table2_id, &db));
    /// assert!(!table2.has_column(table1_id, &db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_column(&self, column: &<Self::DB as DatabaseLike>::Column, database: &Self::DB) -> bool {
        TableLike::columns(self, database).any(|col| col == column)
    }

    /// Iterates over the primary key columns of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// CREATE TABLE my_no_pk_table (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let pk_columns: Vec<&str> =
    ///     table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(pk_columns, vec!["id"]);
    /// let composite_pk_table = db.table(None, "my_composite_pk_table").unwrap();
    /// let composite_pk_columns: Vec<&str> =
    ///     composite_pk_table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(composite_pk_columns, vec!["id1", "id2"]);
    /// let no_pk_table = db.table(None, "my_no_pk_table").unwrap();
    /// let no_pk_columns: Vec<&str> =
    ///     no_pk_table.primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(no_pk_columns, Vec::<&str>::new());
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db;

    /// Returns the single primary key column of the table, if it exists and is
    /// non-composite.
    ///
    /// Returns `None` if the table has no primary key or if the primary key is
    /// composite.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE composite_pk (id1 INT, id2 INT, PRIMARY KEY (id1, id2));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let pk_column = table.primary_key_column(&db).unwrap();
    /// assert_eq!(pk_column.column_name(), "id");
    ///
    /// let composite_table = db.table(None, "composite_pk").unwrap();
    /// assert!(composite_table.primary_key_column(&db).is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_column<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Option<&'db <Self::DB as DatabaseLike>::Column> {
        let mut pk_columns = self.primary_key_columns(database);
        let pk_column = pk_columns.next()?;
        if pk_columns.next().is_some() {
            return None; // Composite primary key
        }
        Some(pk_column)
    }

    /// Returns whether the primary key of the table is generated (i.e.,
    /// auto-incrementing).
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_no_gen_pk_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert!(table.has_surrogate_primary_key(&db));
    /// let no_gen_pk_table = db.table(None, "my_no_gen_pk_table").unwrap();
    /// assert!(!no_gen_pk_table.has_surrogate_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_surrogate_primary_key(&self, database: &Self::DB) -> bool {
        self.primary_key_columns(database).all(ColumnLike::is_generated)
            && self.has_primary_key(database)
    }

    /// Returns a vector with the normalized data types of the primary key
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id SERIAL PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 BIGSERIAL, name TEXT, PRIMARY KEY (id1, id2));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let pk_types = table.primary_key_type(&db);
    /// assert_eq!(pk_types, vec!["INT"]);
    /// let composite_pk_table = db.table(None, "my_composite_pk_table").unwrap();
    /// let composite_pk_types = composite_pk_table.primary_key_type(&db);
    /// assert_eq!(composite_pk_types, vec!["INT", "BIGINT"]);
    /// # Ok(())
    /// # }
    /// ```
    fn primary_key_type<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db str> {
        self.primary_key_columns(database).map(|col| col.normalized_data_type(database)).collect()
    }

    /// Returns whether the provided column is the primary key column of the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `column` - A reference to the column to check.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let id_column = table.column("id", &db).expect("Column 'id' should exist");
    /// let name_column = table.column("name", &db).expect("Column 'name' should exist");
    /// assert!(table.is_primary_key_column(&db, id_column));
    /// assert!(!table.is_primary_key_column(&db, name_column));
    /// # Ok(())
    /// # }
    /// ```
    fn is_primary_key_column(
        &self,
        database: &Self::DB,
        column: &<Self::DB as DatabaseLike>::Column,
    ) -> bool {
        self.primary_key_columns(database).all(|col| col == column)
            && self.has_primary_key(database)
    }

    /// Returns whether the table has a primary key.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_no_pk_table (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert!(table.has_primary_key(&db));
    /// let no_pk_table = db.table(None, "my_no_pk_table").unwrap();
    /// assert!(!no_pk_table.has_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_primary_key(&self, database: &Self::DB) -> bool {
        self.primary_key_columns(database).next().is_some()
    }

    /// Returns an iterator over the non-primary key columns of the table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT, age INT);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let non_pk_columns: Vec<&str> =
    ///     table.non_primary_key_columns(&db).map(|col| col.column_name()).collect();
    /// assert_eq!(non_pk_columns, vec!["name", "age"]);
    /// # Ok(())
    /// # }
    /// ```
    fn non_primary_key_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        let primary_key_column_names: Vec<&<Self::DB as DatabaseLike>::Column> =
            self.primary_key_columns(database).collect();

        self.columns(database).filter(move |col| !primary_key_column_names.contains(col))
    }

    /// Returns whether the table has non-primary key columns.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_pk_only_table (id INT PRIMARY KEY);
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert!(table.has_non_primary_key_columns(&db));
    /// let pk_only_table = db.table(None, "my_pk_only_table").unwrap();
    /// assert!(!pk_only_table.has_non_primary_key_columns(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_non_primary_key_columns(&self, database: &Self::DB) -> bool {
        self.non_primary_key_columns(database).next().is_some()
    }

    /// Returns whether the table has a composite primary key.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE my_composite_pk_table (id1 INT, id2 INT, name TEXT, PRIMARY KEY (id1, id2));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert!(!table.has_composite_primary_key(&db));
    /// let composite_pk_table = db.table(None, "my_composite_pk_table").unwrap();
    /// assert!(composite_pk_table.has_composite_primary_key(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_composite_primary_key(&self, database: &Self::DB) -> bool {
        self.primary_key_columns(database).nth(1).is_some()
    }

    /// Iterates over the check constraints of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT CHECK (id > 0), name TEXT, CHECK (length(name) > 0));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let check_constraints: Vec<_> =
    ///     table.check_constraints(&db).map(|cc| cc.expression(&db).to_string()).collect();
    /// assert_eq!(check_constraints, vec!["id > 0", "length(name) > 0"]);
    /// # Ok(())
    /// # }
    /// ```
    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint>
    where
        Self: 'db;

    /// Iterates over the non-tautological check constraints of the table using
    /// the provided schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT CHECK (TRUE), name TEXT, CHECK (length(name) > 0));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let non_tautological_ccs: Vec<_> = table
    ///     .non_tautological_check_constraints(&db)
    ///     .map(|cc| cc.expression(&db).to_string())
    ///     .collect();
    /// assert_eq!(non_tautological_ccs, vec!["length(name) > 0"]);
    /// # Ok(())
    /// # }
    /// ```
    fn non_tautological_check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint>
    where
        Self: 'db,
    {
        self.check_constraints(database).filter(|cc| !cc.is_tautology(database))
    }

    /// Returns whether the table has any check constraints.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table_with_cc (id INT CHECK (id > 0), name TEXT);
    /// CREATE TABLE my_table_without_cc (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let table_with_cc = db.table(None, "my_table_with_cc").unwrap();
    /// assert!(table_with_cc.has_check_constraints(&db));
    /// let table_without_cc = db.table(None, "my_table_without_cc").unwrap();
    /// assert!(!table_without_cc.has_check_constraints(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_check_constraints(&self, database: &Self::DB) -> bool {
        self.check_constraints(database).next().is_some()
    }

    /// Returns whether the table has any non-tautological check constraints.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table_with_non_tautological_cc (id INT CHECK (id > 0), name TEXT);
    /// CREATE TABLE my_table_with_only_tautological_cc (id INT CHECK (TRUE), name TEXT);
    /// "#,
    /// )?;
    /// let table_with_non_tautological_cc =
    ///     db.table(None, "my_table_with_non_tautological_cc").unwrap();
    /// assert!(table_with_non_tautological_cc.has_non_tautological_check_constraints(&db));
    /// let table_with_only_tautological_cc =
    ///     db.table(None, "my_table_with_only_tautological_cc").unwrap();
    /// assert!(!table_with_only_tautological_cc.has_non_tautological_check_constraints(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_non_tautological_check_constraints(&self, database: &Self::DB) -> bool {
        self.non_tautological_check_constraints(database).next().is_some()
    }

    /// Iterates over the unique indexes of the table using the provided
    /// schema.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_table (id INT UNIQUE, name TEXT, UNIQUE (name));
    /// "#,
    /// )?;
    /// let table = db.table(None, "my_table").unwrap();
    /// let unique_indices: Vec<_> = table
    ///     .unique_indices(&db)
    ///     .map(|ui| ui.columns(&db).map(|col| col.column_name()).collect::<Vec<_>>())
    ///     .collect();
    /// assert_eq!(unique_indices, vec![vec!["id"], vec!["name"]]);
    /// # Ok(())
    /// # }
    /// ```
    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::UniqueIndex>
    where
        Self: 'db;

    /// Iterates over the foreign keys of the table using the provided schema.
    ///
    /// # Arguments
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    /// ```
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT, FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let foreign_keys = host_table.foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(foreign_keys.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db;

    /// Returns whether the table has any foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table_with_fk (id INT, name TEXT, FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// CREATE TABLE host_table_without_fk (id INT, name TEXT);
    /// "#,
    /// )?;
    /// let referenced_table = db.table(None, "referenced_table").unwrap();
    /// assert!(!referenced_table.has_foreign_keys(&db));
    /// let host_table_with_fk = db.table(None, "host_table_with_fk").unwrap();
    /// assert!(host_table_with_fk.has_foreign_keys(&db));
    /// let host_table_without_fk = db.table(None, "host_table_without_fk").unwrap();
    /// assert!(!host_table_without_fk.has_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_foreign_keys(&self, database: &Self::DB) -> bool {
        self.foreign_keys(database).next().is_some()
    }

    /// Returns whether the table has non-self-referential foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, parent_id INT REFERENCES parent_table(id));
    /// CREATE TABLE self_ref_table (id INT PRIMARY KEY, parent_id INT REFERENCES self_ref_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// assert!(child_table.has_non_self_referential_foreign_keys(&db));
    /// let self_ref_table = db.table(None, "self_ref_table").unwrap();
    /// assert!(!self_ref_table.has_non_self_referential_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn has_non_self_referential_foreign_keys(&self, database: &Self::DB) -> bool {
        self.foreign_keys(database).any(move |fk| !fk.is_self_referential(database))
    }

    /// Iterates over the foreign keys in the current table which refer to
    /// ancestors of the provided table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `table` - A reference to the table whose ancestors are to be
    ///   considered.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    /// CREATE TABLE other_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     parent_id INT REFERENCES parent_table(id),
    ///     grandparent_id INT REFERENCES grandparent_table(id),
    ///     other_id INT REFERENCES other_table(id)
    /// );
    /// "#,
    /// )?;
    ///
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let fks_to_ancestors = host_table.foreign_keys_to_ancestors_of(&db, child_table);
    /// assert_eq!(fks_to_ancestors.count(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn foreign_keys_to_ancestors_of<'db>(
        &'db self,
        database: &'db Self::DB,
        table: &'db Self,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        let ancestors = table.ancestral_extended_tables(database);
        self.foreign_keys(database).filter(move |fk| {
            let referenced_table = fk.referenced_table(database).borrow();
            ancestors.iter().any(|ancestor| ancestor == &referenced_table)
                && fk.is_referenced_primary_key(database)
        })
    }

    /// Returns a vector with the (deduplicated) tables which are referenced by
    /// the current table via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table1 (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE referenced_table2 (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table1(id),
    ///     FOREIGN KEY (id) REFERENCES referenced_table2(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let referenced_tables = host_table.referenced_tables(&db);
    /// assert_eq!(referenced_tables.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_tables<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = Vec::new();

        for foreign_key in self.foreign_keys(database) {
            referenced_tables.push(foreign_key.referenced_table(database).borrow());
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        referenced_tables
    }

    /// Returns a vector with the (deduplicated) tables which are referenced by
    /// the current table via foreign keys, and which are not the table itself.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    fn non_self_referenced_tables<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = self.referenced_tables(database);
        referenced_tables.retain(|&table| table != self);
        referenced_tables
    }

    /// Returns the foreign keys which are used to define extensions.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT PRIMARY KEY REFERENCES referenced_table(id), name TEXT);
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let extension_fks = host_table.extension_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(extension_fks.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn extension_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_extension_foreign_key(database))
    }

    /// Returns the tables which are extended by the current table via foreign
    /// keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE extended_table (id INT PRIMARY KEY);
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     other_id INT,
    ///     FOREIGN KEY (other_id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES extended_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let extended_tables = host_table.extended_tables(&db);
    /// assert_eq!(extended_tables.count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn extended_tables<'db>(&'db self, database: &'db Self::DB) -> impl Iterator<Item = &'db Self>
    where
        Self: 'db,
    {
        self.extension_foreign_keys(database).map(|fk| fk.referenced_table(database).borrow())
    }

    /// Returns the root table of the extension hierarchy for the current
    /// table, if any. If the table is not extending any other table, returns
    /// `None`.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let root_table = child_table.extension_root_table(&db).unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// assert_eq!(root_table, grandparent_table);
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let parent_root_table = parent_table.extension_root_table(&db).unwrap();
    /// assert_eq!(parent_root_table, grandparent_table);
    /// let grandparent_root_table = grandparent_table.extension_root_table(&db);
    /// assert!(grandparent_root_table.is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn extension_root_table<'db>(&'db self, database: &'db Self::DB) -> Option<&'db Self>
    where
        Self: 'db,
    {
        if let Some(extension) = self.extended_tables(database).next() {
            extension.extension_root_table(database).or(Some(extension))
        } else {
            None
        }
    }

    /// Returns the tables which extend the current table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY);
    /// CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let extending_tables = parent_table.extending_tables(&db);
    /// assert_eq!(extending_tables.count(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn extending_tables<'db>(&'db self, database: &'db Self::DB) -> impl Iterator<Item = &'db Self>
    where
        Self: 'db,
    {
        database
            .tables()
            .map(Borrow::borrow)
            .filter(move |table: &&Self| table.is_descendant_of(database, self))
    }

    /// Returns whether the current table is extended by any other table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY);
    /// CREATE TABLE child_table (id INT PRIMARY KEY REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// assert!(parent_table.is_extended(&db));
    /// let child_table = db.table(None, "child_table").unwrap();
    /// assert!(!child_table.is_extended(&db));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    fn is_extended(&self, database: &Self::DB) -> bool {
        self.extending_tables(database).next().is_some()
    }

    /// Returns the first extension foreign key found in the current table which
    /// references to the provided table or any of its descendants.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `table` - A reference to the table to check for extensions.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE father_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE mother_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (
    ///     id INT PRIMARY KEY,
    ///     FOREIGN KEY (id) REFERENCES father_table(id),
    ///     FOREIGN KEY (id) REFERENCES mother_table(id)
    /// );
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let father_table = db.table(None, "father_table").unwrap();
    /// let mother_table = db.table(None, "mother_table").unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// let extension_fks = child_table.extension_foreign_keys(&db).collect::<Vec<_>>();
    /// let [father_extension_fk, mother_extension_fk] = extension_fks.as_slice() else {
    ///     panic!("Expected two extension foreign keys");
    /// };
    /// let extension_fk_to_father = child_table.extension_foreign_key_to(&db, father_table);
    /// assert_eq!(extension_fk_to_father, Some(*father_extension_fk));
    /// let extension_fk_to_mother = child_table.extension_foreign_key_to(&db, mother_table);
    /// assert_eq!(extension_fk_to_mother, Some(*mother_extension_fk));
    /// let extension_fk_to_grandparent = child_table.extension_foreign_key_to(&db, grandparent_table);
    /// assert_eq!(extension_fk_to_grandparent, Some(*father_extension_fk));
    /// assert!(child_table.extension_foreign_key_to(&db, child_table).is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn extension_foreign_key_to<'db>(
        &'db self,
        database: &'db Self::DB,
        table: &'db Self,
    ) -> Option<&'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.extension_foreign_keys(database).find(|fk| {
            let referenced_table: &Self = fk.referenced_table(database).borrow();
            referenced_table == table
                || referenced_table.extension_foreign_key_to(database, table).is_some()
        })
    }

    /// Returns the first extended table found in the current table which
    /// matches the provided table or is its descendants.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `table` - A reference to the table to check for extensions.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY);
    /// CREATE TABLE father_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE mother_table (id INT PRIMARY KEY REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (
    ///     id INT PRIMARY KEY,
    ///     FOREIGN KEY (id) REFERENCES father_table(id),
    ///     FOREIGN KEY (id) REFERENCES mother_table(id)
    /// );
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let father_table = db.table(None, "father_table").unwrap();
    /// let mother_table = db.table(None, "mother_table").unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// let extended_table_to_father = child_table.extended_table_to(&db, father_table);
    /// assert_eq!(extended_table_to_father, Some(father_table));
    /// let extended_table_to_mother = child_table.extended_table_to(&db, mother_table);
    /// assert_eq!(extended_table_to_mother, Some(mother_table));
    /// let extended_table_to_grandparent = child_table.extended_table_to(&db, grandparent_table);
    /// assert_eq!(extended_table_to_grandparent, Some(father_table));
    /// assert!(child_table.extended_table_to(&db, child_table).is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn extended_table_to<'db>(
        &'db self,
        database: &'db Self::DB,
        table: &'db Self,
    ) -> Option<&'db Self>
    where
        Self: 'db,
    {
        self.extension_foreign_key_to(database, table)
            .map(|fk| fk.referenced_table(database).borrow())
    }

    /// Returns the unique tables which are extended by either the current
    /// table or any of the tables it extends via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let ancestral_tables = child_table.ancestral_extended_tables(&db);
    /// assert_eq!(ancestral_tables.len(), 2);
    /// # Ok(())
    /// # }
    /// ```
    fn ancestral_extended_tables<'db>(&'db self, database: &'db Self::DB) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let extension_tables = self.extended_tables(database).collect::<Vec<&Self>>();
        let mut ancestral_tables = extension_tables.clone();

        for table in &extension_tables {
            let mut parent_ancestral_tables = table.ancestral_extended_tables(database);
            ancestral_tables.append(&mut parent_ancestral_tables);
        }

        ancestral_tables.sort_unstable();
        ancestral_tables.dedup();

        ancestral_tables
    }

    /// Returns the unique tables which are extended by either the current
    /// table or any of the tables it extends via foreign keys, sorted by
    /// topological order.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let ancestral_tables: Vec<&str> = child_table
    ///     .ancestral_extended_tables_topological(&db)
    ///     .iter()
    ///     .map(|t| t.table_name())
    ///     .collect();
    /// assert_eq!(ancestral_tables, vec!["grandparent_table", "parent_table"]);
    /// # Ok(())
    /// # }
    /// ```
    fn ancestral_extended_tables_topological<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut ancestral_extended_tables = self.ancestral_extended_tables(database);

        if ancestral_extended_tables.len() <= 1 {
            return ancestral_extended_tables;
        }

        let sorted_dag =
            database.table_dag().into_iter().map(|table| table.borrow()).collect::<Vec<&Self>>();

        ancestral_extended_tables.sort_by_key(|table| {
            sorted_dag
                .iter()
                .position(|t| t == table)
                .expect("Table in ancestral extended tables should exist in the DAG")
        });

        ancestral_extended_tables
    }

    /// Returns the tables referenced in foreign keys of the current table via
    /// the provided column.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `column` - A reference to the column in the current table.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let id_column = host_table.column("id", &db).expect("Column 'id' should exist");
    /// let referenced_tables = host_table.referenced_tables_via_column(&db, id_column);
    /// assert_eq!(referenced_tables.len(), 1);
    /// let name_column = host_table.column("name", &db).expect("Column 'name' should exist");
    /// let no_referenced_tables = host_table.referenced_tables_via_column(&db, name_column);
    /// assert_eq!(no_referenced_tables.len(), 0);
    /// # Ok(())
    /// # }
    /// ```
    fn referenced_tables_via_column<'db>(
        &'db self,
        database: &'db Self::DB,
        column: &<Self::DB as DatabaseLike>::Column,
    ) -> Vec<&'db Self>
    where
        Self: 'db,
    {
        let mut referenced_tables = Vec::new();

        for fk in self.foreign_keys(database) {
            if fk.host_columns(database).all(|col| col == column)
                && fk.is_referenced_primary_key(database)
            {
                referenced_tables.push(fk.referenced_table(database).borrow());
            }
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        referenced_tables
    }

    /// Returns whether the table extends any other table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// assert!(child_table.is_extension(&db));
    /// assert!(!parent_table.is_extension(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn is_extension(&self, database: &Self::DB) -> bool {
        self.extension_foreign_keys(database).next().is_some()
    }

    /// Returns whether the table is a descendant of another table, i.e., if it
    /// extends the other table either directly or some other table which
    /// extends the other table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// assert!(child_table.is_descendant_of(&db, parent_table));
    /// assert!(!parent_table.is_descendant_of(&db, child_table));
    /// # Ok(())
    /// # }
    /// ```
    fn is_descendant_of(&self, database: &Self::DB, other: &Self) -> bool {
        self.ancestral_extended_tables(database).contains(&other)
    }

    /// Returns whether the table shares any ancestor with the given table.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// CREATE TABLE unrelated_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// let unrelated_table = db.table(None, "unrelated_table").unwrap();
    /// assert!(
    ///     child_table.shares_ancestors_with(&db, parent_table),
    ///     "Child should share ancestors with parent"
    /// );
    /// assert!(
    ///     child_table.shares_ancestors_with(&db, grandparent_table),
    ///     "Child should share ancestors with grandparent"
    /// );
    /// assert!(
    ///     !child_table.shares_ancestors_with(&db, unrelated_table),
    ///     "Child should not share ancestors with unrelated"
    /// );
    /// # Ok(())
    /// # }
    /// ```
    fn shares_ancestors_with(&self, database: &Self::DB, other: &Self) -> bool {
        let self_ancestors = self.ancestral_extended_tables(database);
        let other_ancestors = other.ancestral_extended_tables(database);

        self_ancestors.iter().any(|table| other_ancestors.contains(table))
            || self == other
            || self_ancestors.contains(&other)
            || other_ancestors.contains(&self)
    }

    /// Returns the table singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT UNIQUE, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let singleton_fks = host_table.singleton_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(singleton_fks.len(), 1);
    /// # Ok(())
    /// # }
    /// ```
    fn singleton_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.foreign_keys(database).filter(|fk| fk.is_singleton(database))
    }

    /// Returns the table singleton foreign keys which are not self-referential.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES host_table(id)
    /// );
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// let non_self_referential_singleton_fks =
    ///     host_table.non_self_referential_singleton_foreign_keys(&db).collect::<Vec<_>>();
    /// assert_eq!(non_self_referential_singleton_fks.len(), 1);
    /// assert!(non_self_referential_singleton_fks[0].is_singleton(&db));
    /// assert!(!non_self_referential_singleton_fks[0].is_self_referential(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn non_self_referential_singleton_foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        self.singleton_foreign_keys(database).filter(move |fk| !fk.is_self_referential(database))
    }

    /// Returns whether the table has singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (id INT UNIQUE, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id));
    /// "#,
    /// )?;
    /// let host_table = db.table(None, "host_table").unwrap();
    /// assert!(host_table.has_singleton_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_singleton_foreign_keys(&self, database: &Self::DB) -> bool {
        self.singleton_foreign_keys(database).next().is_some()
    }

    /// Returns whether the table has non-self-referential singleton foreign
    /// keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE referenced_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE host_table (
    ///     id INT PRIMARY KEY,
    ///     name TEXT,
    ///     FOREIGN KEY (id) REFERENCES referenced_table(id),
    ///     FOREIGN KEY (id) REFERENCES host_table(id)
    /// );
    /// "#,
    /// )?;
    /// let referenced_table = db.table(None, "referenced_table").unwrap();
    /// assert!(!referenced_table.has_non_self_referential_singleton_foreign_keys(&db));
    /// let host_table = db.table(None, "host_table").unwrap();
    /// assert!(host_table.has_non_self_referential_singleton_foreign_keys(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_non_self_referential_singleton_foreign_keys(&self, database: &Self::DB) -> bool {
        self.non_self_referential_singleton_foreign_keys(database).next().is_some()
    }

    /// Returns whether the table depends directly or indirectly on another
    /// table via foreign keys.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   and the other table belong.
    /// * `other` - The other table to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let child_table = db.table(None, "child_table").unwrap();
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// assert!(child_table.depends_on(&db, parent_table));
    /// assert!(child_table.depends_on(&db, grandparent_table));
    /// assert!(!parent_table.depends_on(&db, child_table));
    /// assert!(!grandparent_table.depends_on(&db, child_table));
    /// assert!(parent_table.depends_on(&db, grandparent_table));
    /// assert!(!grandparent_table.depends_on(&db, parent_table));
    /// assert!(child_table.depends_on(&db, child_table));
    /// assert!(parent_table.depends_on(&db, parent_table));
    /// assert!(grandparent_table.depends_on(&db, grandparent_table));
    /// # Ok(())
    /// # }
    /// ```
    fn depends_on(&self, database: &Self::DB, other: &Self) -> bool {
        if self == other {
            return true;
        }
        self.foreign_keys(database).any(|fk| {
            let referenced_table: &Self = fk.referenced_table(database).borrow();
            referenced_table == other
                || referenced_table != self && referenced_table.depends_on(database, other)
        })
    }

    /// Returns an iterator over all tables that depend directly or indirectly
    /// via foreign keys (including extensions) on the current table, excluding
    /// itself.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// let dependent_tables: Vec<&str> =
    ///     grandparent_table.dependent_tables(&db).map(|t| t.table_name()).collect();
    /// assert_eq!(dependent_tables, vec!["child_table", "parent_table"]);
    /// # Ok(())
    /// # }
    /// ```
    fn dependent_tables<'db>(&'db self, database: &'db Self::DB) -> impl Iterator<Item = &'db Self>
    where
        Self: 'db,
    {
        database.tables().filter_map(move |table| {
            let table_ref: &Self = table.borrow();
            if table_ref != self && table_ref.depends_on(database, self) {
                Some(table_ref)
            } else {
                None
            }
        })
    }

    /// Returns whether the table has any dependent tables.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    ///
    /// # Example
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE child_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// "#,
    /// )?;
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let child_table = db.table(None, "child_table").unwrap();
    /// assert!(parent_table.has_dependent_tables(&db));
    /// assert!(!child_table.has_dependent_tables(&db));
    /// # Ok(())
    /// # }
    /// ```
    fn has_dependent_tables(&self, database: &Self::DB) -> bool {
        self.dependent_tables(database).next().is_some()
    }

    /// Returns the most recent common ancestor table between the current table
    /// and all of the provided tables, if any.
    ///
    /// # Arguments
    ///
    /// * `database` - A reference to the database instance to which the table
    ///   belongs.
    /// * `others` - A slice of other tables to check against.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
    /// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES grandparent_table(id));
    /// CREATE TABLE child_table1 (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// CREATE TABLE child_table2 (id INT PRIMARY KEY, name TEXT,
    ///     FOREIGN KEY (id) REFERENCES parent_table(id));
    /// CREATE TABLE unrelated_table (id INT PRIMARY KEY, name TEXT);
    /// "#,
    /// )?;
    /// let child_table1 = db.table(None, "child_table1").unwrap();
    /// let child_table2 = db.table(None, "child_table2").unwrap();
    /// let parent_table = db.table(None, "parent_table").unwrap();
    /// let grandparent_table = db.table(None, "grandparent_table").unwrap();
    /// let unrelated_table = db.table(None, "unrelated_table").unwrap();
    /// assert_eq!(child_table1.most_recent_common_ancestor(&db, &[child_table2]), Some(parent_table));
    /// assert_eq!(child_table1.most_recent_common_ancestor(&db, &[parent_table]), Some(parent_table));
    /// assert_eq!(
    ///     child_table1.most_recent_common_ancestor(&db, &[grandparent_table]),
    ///     Some(grandparent_table)
    /// );
    /// assert_eq!(child_table1.most_recent_common_ancestor(&db, &[unrelated_table]), None);
    /// assert_eq!(
    ///     child_table1.most_recent_common_ancestor(&db, &[child_table2, parent_table]),
    ///     Some(parent_table)
    /// );
    /// assert_eq!(
    ///     child_table1.most_recent_common_ancestor(&db, &[child_table2, grandparent_table]),
    ///     Some(grandparent_table)
    /// );
    /// assert_eq!(child_table1.most_recent_common_ancestor(&db, &[]), Some(child_table1));
    /// # Ok(())
    /// # }
    /// ```
    fn most_recent_common_ancestor<'db>(
        &'db self,
        database: &'db Self::DB,
        others: &[&'db Self],
    ) -> Option<&'db Self>
    where
        Self: 'db,
    {
        if others.iter().all(|&other| other == self || other.is_descendant_of(database, self)) {
            return Some(self);
        }

        for extended_table in self.extended_tables(database) {
            if let Some(common_ancestor) =
                extended_table.most_recent_common_ancestor(database, others)
            {
                return Some(common_ancestor);
            }
        }

        None
    }
}

impl<T: TableLike> TableLike for &T
where
    Self: Borrow<<<T as TableLike>::DB as DatabaseLike>::Table>,
    for<'a> <T::DB as DatabaseLike>::Table: Borrow<&'a T>,
{
    type DB = T::DB;

    fn table_name(&self) -> &str {
        T::table_name(self)
    }

    fn table_doc<'db>(&'db self, database: &'db Self::DB) -> Option<&'db str>
    where
        Self: 'db,
    {
        T::table_doc(self, database)
    }

    fn table_schema(&self) -> Option<&str> {
        T::table_schema(self)
    }

    fn columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        T::columns(self, database)
    }

    fn primary_key_columns<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::Column>
    where
        Self: 'db,
    {
        T::primary_key_columns(self, database)
    }

    fn check_constraints<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::CheckConstraint>
    where
        Self: 'db,
    {
        T::check_constraints(self, database)
    }

    fn unique_indices<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::UniqueIndex>
    where
        Self: 'db,
    {
        T::unique_indices(self, database)
    }

    fn foreign_keys<'db>(
        &'db self,
        database: &'db Self::DB,
    ) -> impl Iterator<Item = &'db <Self::DB as DatabaseLike>::ForeignKey>
    where
        Self: 'db,
    {
        T::foreign_keys(self, database)
    }
}
