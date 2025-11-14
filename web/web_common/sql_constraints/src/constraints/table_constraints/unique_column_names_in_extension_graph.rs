//! Submodule providing the `UniqueColumnNamesInExtensionGraph` constraint,
//! which enforces that column names are unique within an extension graph.

use std::collections::HashSet;

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that column names are unique
/// within an extension graph (i.e., a table which extends other tables cannot
/// have a column whose name also appears in other tables it extends).
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `UniqueColumnNamesInExtensionGraph` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> =
///     UniqueColumnNamesInExtensionGraph::default().into();
///
/// // Invalid: child_table has 'name' column, which also exists in parent_table
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     name TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// // Valid: child_table has unique column names
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     description TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Valid: non-extension tables can have any column names
/// let valid_non_extension = ParserDB::try_from(
///     r#"
/// CREATE TABLE table1 (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE table2 (id INT PRIMARY KEY, name TEXT);
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_non_extension).is_ok());
///
/// // Valid: primary key column (id) can be shared in extension
/// let valid_shared_pk = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     value INT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_shared_pk).is_ok());
///
/// // Invalid: transitive extension - grandchild cannot have column from grandparent
/// let invalid_transitive = ParserDB::try_from(
///     r#"
/// CREATE TABLE grandparent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE parent_table (
///     id INT PRIMARY KEY,
///     age INT,
///     FOREIGN KEY (id) REFERENCES grandparent_table(id)
/// );
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     name TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_transitive).is_err());
/// ```
pub struct UniqueColumnNamesInExtensionGraph<DB>(std::marker::PhantomData<DB>);

impl<DB> Default for UniqueColumnNamesInExtensionGraph<DB> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<DB: DatabaseLike + 'static> From<UniqueColumnNamesInExtensionGraph<DB>>
    for GenericConstrainer<DB>
{
    fn from(constraint: UniqueColumnNamesInExtensionGraph<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> UniqueColumnNamesInExtensionGraph<DB> {
    /// Helper method to find duplicate column names in the extension graph
    fn find_duplicate_columns(database: &DB, table: &<DB as DatabaseLike>::Table) -> Vec<String> {
        let mut duplicates = Vec::new();
        let extended_tables = table.extended_tables(database);

        if extended_tables.is_empty() {
            return duplicates;
        }

        let mut extended_column_names: HashSet<String> = HashSet::new();
        let mut tables_to_check = extended_tables.clone();
        let mut checked_tables: HashSet<String> = HashSet::new();

        while let Some(extended_table) = tables_to_check.pop() {
            let table_name = extended_table.table_name().to_string();
            if checked_tables.contains(&table_name) {
                continue;
            }
            checked_tables.insert(table_name);

            for column in extended_table.columns(database) {
                extended_column_names.insert(column.column_name().to_string());
            }

            for parent in extended_table.extended_tables(database) {
                tables_to_check.push(parent);
            }
        }

        for column in table.columns(database) {
            let column_name = column.column_name();
            if table.primary_key_columns(database).any(|pk_col| pk_col.column_name() == column_name)
            {
                continue;
            }
            if extended_column_names.contains(column_name) {
                duplicates.push(column_name.to_string());
            }
        }

        duplicates
    }
}

impl<DB: DatabaseLike> TableConstraint for UniqueColumnNamesInExtensionGraph<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        // Note: We can't compute duplicates here as we don't have access to the
        // database The error message will be generic
        ConstraintErrorInfo::new()
            .constraint("UniqueColumnNamesInExtensionGraph")
            .unwrap()
            .object(context.table_name().to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' has duplicate column name(s) in its extension hierarchy",
                context.table_name()
            ))
            .unwrap()
            .resolution(format!(
                "Ensure all columns in table '{}' have unique names that don't conflict with any parent table columns (excluding primary key columns)",
                context.table_name()
            ))
            .unwrap()
            .build()
            .unwrap()
            .into()
    }

    fn validate_table(
        &self,
        database: &Self::Database,
        table: &<Self::Database as DatabaseLike>::Table,
    ) -> Result<(), crate::error::Error> {
        // Check if the table extends other tables
        let extended_tables = table.extended_tables(database);

        // If there are no extended tables, no constraint applies
        if extended_tables.is_empty() {
            return Ok(());
        }

        // Collect all column names from extended tables (recursively)
        let mut extended_column_names: HashSet<String> = HashSet::new();
        let mut tables_to_check = extended_tables.clone();
        let mut checked_tables: HashSet<String> = HashSet::new();

        while let Some(extended_table) = tables_to_check.pop() {
            let table_name = extended_table.table_name().to_string();

            // Avoid infinite loops in case of circular dependencies
            if checked_tables.contains(&table_name) {
                continue;
            }
            checked_tables.insert(table_name);

            // Add column names from this extended table
            for column in extended_table.columns(database) {
                extended_column_names.insert(column.column_name().to_string());
            }

            // Add the parent tables of this extended table to check
            for parent in extended_table.extended_tables(database) {
                tables_to_check.push(parent);
            }
        }

        // Check if any column in the current table conflicts with extended tables
        let duplicates = Self::find_duplicate_columns(database, table);

        if !duplicates.is_empty() {
            let duplicate_list = duplicates.join(", ");
            return Err(crate::error::Error::Table(
                ConstraintErrorInfo::new()
                    .constraint("UniqueColumnNamesInExtensionGraph")
                    .unwrap()
                    .object(table.table_name().to_owned())
                    .unwrap()
                    .message(format!(
                        "Table '{}' has {} duplicate column name(s) that conflict with parent tables: {}",
                        table.table_name(),
                        duplicates.len(),
                        duplicate_list
                    ))
                    .unwrap()
                    .resolution(format!(
                        "Rename these columns in table '{}': [{}]. Each column must have a unique name across all parent tables in the extension hierarchy",
                        table.table_name(),
                        duplicate_list
                    ))
                    .unwrap()
                    .build()
                    .unwrap()
                    .into(),
            ));
        }

        Ok(())
    }
}
