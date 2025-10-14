//! Struct collecting information about a
//! [`KeyColumnUsage`](crate::models::KeyColumnUsage) entry's referenced table
//! and columns.

use diesel::PgConnection;

use crate::models::{Column, KeyColumnUsage, ReferentialConstraint, Table};

#[derive(Debug, Clone)]
/// Struct collecting metadata about a foreign key represented by a
/// [`KeyColumnUsage`](crate::models::KeyColumnUsage) entry.
pub struct KeyColumnUsageMetadata {
    /// The table that the foreign key references.
    referenced_table: Table,
    /// The columns in the referenced table that the foreign key points to.
    referenced_columns: Vec<Column>,
    /// The table that contains the foreign key.
    host_table: Table,
    /// The columns in the host table that are part of the foreign key.
    host_columns: Vec<Column>,
    /// The referential constraint associated with the foreign key.
    referential_constraint: ReferentialConstraint,
}

impl KeyColumnUsageMetadata {
    /// Creates a new `KeyColumnUsageMetadata` instance from the given
    /// [`KeyColumnUsage`](crate::models::KeyColumnUsage), and connection to the
    /// database.
    ///
    /// # Arguments
    ///
    /// * `key_column_usage` - The `KeyColumnUsage` instance for which to create
    ///   the metadata.
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// Returns a [`diesel::result::Error`] if the database query fails.
    pub fn new(
        key_column_usage: &KeyColumnUsage,
        conn: &mut PgConnection,
    ) -> Result<Self, diesel::result::Error> {
        Ok(KeyColumnUsageMetadata {
            referenced_table: crate::models::key_column_usage::referenced_table(
                key_column_usage,
                conn,
            )?,
            referenced_columns: crate::models::key_column_usage::referenced_columns(
                key_column_usage,
                conn,
            )?,
            host_table: crate::models::key_column_usage::host_table(key_column_usage, conn)?,
            host_columns: crate::models::key_column_usage::host_columns(key_column_usage, conn)?,
            referential_constraint: crate::models::key_column_usage::referential_constraint(
                key_column_usage,
                conn,
            )?,
        })
    }

    /// Returns a reference to the table that the foreign key references.
    pub fn referenced_table(&self) -> &Table {
        &self.referenced_table
    }

    /// Returns a reference to the table that contains the foreign key.
    pub fn host_table(&self) -> &Table {
        &self.host_table
    }

    /// Returns a reference to the columns in the referenced table that the
    /// foreign key points to.
    pub fn referenced_columns(&self) -> &[Column] {
        &self.referenced_columns
    }

    /// Returns a reference to the columns in the host table that are part of
    /// the foreign key.
    pub fn host_columns(&self) -> &[Column] {
        &self.host_columns
    }

    /// Returns a reference to the referential constraint associated with the
    /// foreign key.
    pub fn referential_constraint(&self) -> &ReferentialConstraint {
        &self.referential_constraint
    }

    /// Returns whether the foreign key has an ON DELETE CASCADE rule.
    pub fn on_delete_cascade(&self) -> bool {
        self.referential_constraint.on_delete_cascade()
    }

    /// Returns the match kind of the foreign key.
    pub fn match_kind(&self) -> sqlparser::ast::MatchKind {
        self.referential_constraint.match_kind()
    }
}
