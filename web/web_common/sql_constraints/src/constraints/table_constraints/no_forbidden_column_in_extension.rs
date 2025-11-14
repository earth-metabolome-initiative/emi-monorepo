//! Submodule providing the `NoForbiddenColumnInExtension` constraint, which
//! enforces that if a table extends other tables, it cannot have a column with
//! a forbidden name (by default "extension", but configurable).

use common_traits::builder::Builder;
use sql_traits::traits::{ColumnLike, DatabaseLike, TableLike};

use crate::{
    error::ConstraintErrorInfo,
    traits::{Constrainer, GenericConstrainer, TableConstraint},
};

/// Struct defining a constraint that enforces that if a table extends other
/// tables, it cannot have a column with a forbidden name (by default
/// "extension", but configurable).
///
/// # Example
///
/// Here follows an example of validating an invalid SQL statement with the
/// `NoForbiddenColumnInExtension` constraint.
///
/// ```rust
/// use sql_constraints::prelude::*;
///
/// let constrainer: GenericConstrainer<ParserDB> = NoForbiddenColumnInExtension::default().into();
///
/// let invalid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     extension TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&invalid_schema).is_err());
///
/// let valid_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     data TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(constrainer.validate_schema(&valid_schema).is_ok());
///
/// // Tables that don't extend other tables can have columns with the forbidden name
/// let valid_non_extension_schema =
///     ParserDB::try_from("CREATE TABLE mytable (id INT PRIMARY KEY, extension TEXT);").unwrap();
/// assert!(constrainer.validate_schema(&valid_non_extension_schema).is_ok());
///
/// // Custom forbidden name
/// let custom_constrainer: GenericConstrainer<ParserDB> =
///     NoForbiddenColumnInExtension::new("custom_forbidden").into();
///
/// let invalid_custom_schema = ParserDB::try_from(
///     r#"
/// CREATE TABLE parent_table (id INT PRIMARY KEY, name TEXT);
/// CREATE TABLE child_table (
///     id INT PRIMARY KEY,
///     custom_forbidden TEXT,
///     FOREIGN KEY (id) REFERENCES parent_table(id)
/// );
/// "#,
/// )
/// .unwrap();
/// assert!(custom_constrainer.validate_schema(&invalid_custom_schema).is_err());
/// ```
pub struct NoForbiddenColumnInExtension<DB> {
    forbidden_name: String,
    _phantom: std::marker::PhantomData<DB>,
}

impl<DB> NoForbiddenColumnInExtension<DB> {
    /// Creates a new `NoForbiddenColumnInExtension` constraint with a custom
    /// forbidden column name.
    ///
    /// # Arguments
    ///
    /// * `forbidden_name` - The name of the column that should not appear in
    ///   extension tables.
    ///
    /// # Example
    ///
    /// ```rust
    /// use sql_constraints::prelude::*;
    ///
    /// let constraint: NoForbiddenColumnInExtension<ParserDB> =
    ///     NoForbiddenColumnInExtension::new("my_forbidden_column");
    /// ```
    pub fn new(forbidden_name: impl Into<String>) -> Self {
        Self { forbidden_name: forbidden_name.into(), _phantom: std::marker::PhantomData }
    }
}

impl<DB> Default for NoForbiddenColumnInExtension<DB> {
    fn default() -> Self {
        Self::new("extension")
    }
}

impl<DB: DatabaseLike + 'static> From<NoForbiddenColumnInExtension<DB>> for GenericConstrainer<DB> {
    fn from(constraint: NoForbiddenColumnInExtension<DB>) -> Self {
        let mut constrainer = GenericConstrainer::default();
        constrainer.register_table_constraint(Box::new(constraint));
        constrainer
    }
}

impl<DB: DatabaseLike> TableConstraint for NoForbiddenColumnInExtension<DB> {
    type Database = DB;

    fn table_error_information(
        &self,
        database: &Self::Database,
        context: &<Self::Database as DatabaseLike>::Table,
    ) -> Box<dyn crate::prelude::ConstraintFailureInformation> {
        let table_name = context.table_name();
        let extended_tables = context.extended_tables(database);
        let extended_table_names: Vec<_> = extended_tables.iter().map(|t| t.table_name()).collect();

        ConstraintErrorInfo::new()
            .constraint("NoForbiddenColumnInExtension")
            .unwrap()
            .object(table_name.to_owned())
            .unwrap()
            .message(format!(
                "Table '{}' extends {} ({}) but has a forbidden column named '{}'",
                table_name,
                if extended_table_names.len() == 1 { "table" } else { "tables" },
                extended_table_names.join(", "),
                self.forbidden_name
            ))
            .unwrap()
            .resolution(format!(
                "Rename or remove the '{}' column from table '{}' (extension tables should not define this column)",
                self.forbidden_name, table_name
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
        if extended_tables.is_empty() {
            // If the table doesn't extend any other table, the constraint doesn't apply
            return Ok(());
        }

        // Check if any column has the forbidden name (case-insensitive comparison)
        let forbidden_name_lower = self.forbidden_name.to_lowercase();
        for column in table.columns(database) {
            let column_name_lower = column.column_name().to_lowercase();
            if column_name_lower == forbidden_name_lower {
                return Err(crate::error::Error::Table(
                    self.table_error_information(database, table),
                ));
            }
        }

        Ok(())
    }
}
