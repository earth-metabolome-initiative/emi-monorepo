//! Submodule providing a trait for describing SQL Database-like entities.

use std::{borrow::Borrow, fmt::Debug};

use algebra::{
    impls::CSR2D,
    prelude::{Kahn, SquareCSR2D},
};
use common_traits::builder::Builder;
use graph::{prelude::GenericEdgesBuilder, traits::EdgesBuilder};

use crate::traits::{
    CheckConstraintLike, ColumnLike, ForeignKeyLike, FunctionLike, TableLike, UniqueIndexLike,
};

/// A trait for types that can be treated as SQL databases.
pub trait DatabaseLike: Clone + Debug {
    /// Type of the tables in the schema.
    type Table: TableLike<DB = Self>;
    /// Type of the columns in the schema.
    type Column: ColumnLike<DB = Self>;
    /// Type of the foreign keys in the schema.
    type ForeignKey: ForeignKeyLike<DB = Self>;
    /// Type of the functions in the schema.
    type Function: FunctionLike;
    /// Type of the unique indexes in the schema.
    type UniqueIndex: UniqueIndexLike<DB = Self>;
    /// Type of the check constraints in the schema.
    type CheckConstraint: CheckConstraintLike;

    /// Returns the name of the database.
    fn catalog_name(&self) -> &str;

    /// Returns the number of tables in the database.
    fn number_of_tables(&self) -> usize;

    /// Iterates over the tables defined in the schema.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE table1 (id INT);
    /// CREATE TABLE table2 (name TEXT);
    /// CREATE TABLE table3 (score DECIMAL);
    /// "#,
    /// )?;
    /// let table_names: Vec<&str> = db.tables().map(|t| t.table_name()).collect();
    /// assert_eq!(table_names, vec!["table1", "table2", "table3"]);
    /// # Ok(())
    /// # }
    /// ```
    fn tables(&self) -> impl Iterator<Item = &Self::Table>;

    /// Returns tables as a Kahn's ordering based on foreign key dependencies,
    /// ignoring potential self-references which would create cycles.
    fn table_dag(&self) -> Vec<&Self::Table> {
        let tables = self.tables().collect::<Vec<&Self::Table>>();
        let mut edges = tables
            .iter()
            .enumerate()
            .flat_map(|(table_number, table)| {
                let tables_ref = tables.as_slice();
                table
                    .foreign_keys(self)
                    .map(Borrow::borrow)
                    .filter_map(move |fk| {
                        let referenced_table = fk.referenced_table(self).borrow();
                        // We ignore self-references to avoid cycles in the DAG.
                        if referenced_table == *table {
                            return None;
                        }
                        tables_ref.binary_search(&referenced_table).ok()
                    })
                    .map(move |referenced_table_number| (referenced_table_number, table_number))
            })
            .collect::<Vec<(usize, usize)>>();

        // There is no guarantee that the foreign keys in a table are ordered,
        // so it is necessary to sort and deduplicate the edges.
        edges.sort_unstable();
        // Furthermore, there is no guarantee that there are no foreign keys
        // referencing the same table, so we deduplicate the edges as well.
        edges.dedup();

        let dag: SquareCSR2D<CSR2D<usize, usize, usize>> = GenericEdgesBuilder::default()
            .expected_shape(tables.len())
            .edges(edges)
            .build()
            .expect("Failed to build table dependency DAG");
        let dag_ordering = dag.kahn().expect("Failed to compute Kahn's ordering");

        let mut ordered_tables = Vec::with_capacity(tables.len());
        for table_index in dag_ordering {
            ordered_tables.push(tables[table_index]);
        }
        ordered_tables
    }

    /// Iterates over the functions created in the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// CREATE FUNCTION greet(name TEXT) RETURNS TEXT AS 'SELECT "Hello, " || name;';
    /// "#,
    /// )?;
    /// let function_names: Vec<&str> = db.functions().map(|f| f.name()).collect();
    /// assert_eq!(function_names, vec!["add_one", "greet"]);
    /// # Ok(())
    /// # }
    /// ```
    fn functions(&self) -> impl Iterator<Item = &Self::Function>;

    /// Returns the table with the given (optional) schema and name.
    ///
    /// # Arguments
    ///
    /// * `schema` - Optional schema name of the table.
    /// * `table_name` - Name of the table.
    ///
    /// # Panics
    ///
    /// Panics if the table is not found in the database.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE TABLE my_schema.my_table_with_schema (id INT);
    /// CREATE TABLE my_table (id INT);
    /// "#,
    /// )?;
    /// let table_with_schema = db.table(Some("my_schema"), "my_table_with_schema");
    /// assert_eq!(table_with_schema.table_name(), "my_table_with_schema");
    /// assert_eq!(table_with_schema.table_schema(), Some("my_schema"));
    ///
    /// let table_without_schema = db.table(None, "my_table");
    /// assert_eq!(table_without_schema.table_name(), "my_table");
    /// assert_eq!(table_without_schema.table_schema(), None);
    /// # Ok(())
    /// # }
    /// ```
    fn table(&self, schema: Option<&str>, table_name: &str) -> &Self::Table;

    /// Returns the function with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - Name of the function.
    ///
    /// # Example
    ///
    /// ```rust
    /// #  fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use sql_traits::prelude::*;
    ///
    /// let db = ParserDB::try_from(
    ///     r#"
    /// CREATE FUNCTION add_one(x INT) RETURNS INT AS 'SELECT x + 1;';
    /// "#,
    /// )?;
    /// let add_one = db.function("add_one").expect("Function 'add_one' should exist");
    /// assert_eq!(add_one.name(), "add_one");
    /// let non_existent = db.function("non_existent");
    /// assert!(non_existent.is_none());
    /// # Ok(())
    /// # }
    /// ```
    fn function(&self, name: &str) -> Option<&Self::Function>;
}
