//! Struct collecting information about a [`Table`](crate::models::Table)
//! entry's metadata.

use diesel::PgConnection;

use crate::models::{CheckConstraint, Column, KeyColumnUsage, PgIndex};

/// Struct collecting metadata about a table represented by a
/// [`Table`](crate::models::Table) entry.
pub struct TableMetadata {
    /// The columns in the table.
    columns: Vec<Column>,
    /// The foreign keys in the table.
    foreign_keys: Vec<KeyColumnUsage>,
    /// The unique indexes in the table.
    unique_indices: Vec<PgIndex>,
    /// Primary key columns.
    primary_key_columns: Vec<Column>,
    /// Check constraints in the table.
    check_constraints: Vec<CheckConstraint>,
}

impl TableMetadata {
    /// Creates a new `TableMetadata` instance from the given
    /// [`Table`](crate::models::Table), and connection to the database.
    ///
    /// # Arguments
    ///
    /// * `table` - The `Table` instance for which to create the metadata.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// Returns a [`diesel::result::Error`] if the database query fails.
    pub fn new(
        table: &crate::models::Table,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        Ok(TableMetadata {
            columns: crate::models::table::columns(table, conn)?,
            foreign_keys: crate::models::table::foreign_keys(table, conn)?,
            unique_indices: crate::models::table::unique_indices(table, conn)?,
            primary_key_columns: crate::models::table::primary_key_columns(table, conn)?,
            check_constraints: crate::models::table::check_constraints(table, conn)?,
        })
    }

    /// Returns a reference to the columns in the table.
    pub fn columns(&self) -> &[Column] {
        &self.columns
    }

    /// Returns a reference to the foreign keys in the table.
    pub fn foreign_keys(&self) -> &[KeyColumnUsage] {
        &self.foreign_keys
    }

    /// Returns a reference to the unique indexes in the table.
    pub fn unique_indices(&self) -> &[PgIndex] {
        &self.unique_indices
    }

    /// Returns a reference to the primary key columns in the table.
    pub fn primary_key_columns(&self) -> &[Column] {
        &self.primary_key_columns
    }

    /// Returns a reference to the check constraints in the table.
    pub fn check_constraints(&self) -> &[CheckConstraint] {
        &self.check_constraints
    }
}
