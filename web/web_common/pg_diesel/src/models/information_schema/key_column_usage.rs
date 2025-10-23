//! Models for the `information_schema.key_column_usage` table.

use std::rc::Rc;

use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

mod cached_queries;
pub(crate) use cached_queries::*;

use crate::{database::KeyColumnUsageMetadata, models::Table};

/// Represents a row in the `key_column_usage` table, which contains information
/// about columns that are constrained by a unique or primary key constraint.
///
/// For more details, see [`PostgreSQL`](https://www.postgresql.org/docs/current/infoschema-key-column-usage.html)
#[derive(
    Queryable, QueryableByName, Selectable, Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::key_column_usage::key_column_usage)]
pub struct KeyColumnUsage {
    /// The name of the database that contains the constraint.
    pub constraint_catalog: String,
    /// The name of the schema that contains the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The name of the database that contains the table.
    pub table_catalog: String,
    /// The name of the schema that contains the table.
    pub table_schema: String,
    /// The name of the table that contains the column.
    pub table_name: String,
    /// The name of the column that is constrained.
    pub column_name: String,
    /// The position of the column within the constraint.
    pub ordinal_position: i32,
    /// The position of the column within the unique constraint, if applicable.
    pub position_in_unique_constraint: Option<i32>,
}

impl KeyColumnUsage {
    /// Creates a new `KeyColumnUsageMetadata` instance from the given
    /// [`KeyColumnUsage`](crate::models::KeyColumnUsage), and connection to the
    /// database.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a PostgreSQL connection.
    ///
    /// # Errors
    ///
    /// Returns a [`diesel::result::Error`] if the database query fails.
    pub fn metadata(
        &self,
        host_table: Rc<Table>,
        conn: &mut PgConnection,
    ) -> Result<KeyColumnUsageMetadata, diesel::result::Error> {
        Ok(KeyColumnUsageMetadata::new(
            referenced_table(self, conn)?,
            referenced_columns(self, conn)?,
            host_table,
            host_columns(self, conn)?,
            referential_constraint(self, conn)?,
        ))
    }
}
