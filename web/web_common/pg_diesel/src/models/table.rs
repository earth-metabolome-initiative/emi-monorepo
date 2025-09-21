use std::fmt::Display;

use diesel::{PgConnection, Queryable, QueryableByName, Selectable};

use crate::models::{CheckConstraint, Column, KeyColumnUsage, PgIndex, PgTrigger};

mod cached_queries;

#[derive(
    Queryable, QueryableByName, PartialEq, Eq, PartialOrd, Ord, Selectable, Debug, Clone, Hash,
)]
#[diesel(table_name = crate::schema::tables)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Struct defining the `information_schema.tables` table.
pub struct Table {
    /// The catalog name of the table.
    pub table_catalog: String,
    /// The schema name of the table.
    pub table_schema: String,
    /// The name of the table.
    pub table_name: String,
    /// The type of the table, e.g. `BASE TABLE` or `VIEW`.
    pub table_type: String,
    /// The name of the table that the current table is a temporary table for.
    pub self_referencing_column_name: Option<String>,
    /// The name of the column that is a foreign key to the current table.
    pub reference_generation: Option<String>,
    /// The user-defined type catalog.
    pub user_defined_type_catalog: Option<String>,
    /// The user-defined type schema.
    pub user_defined_type_schema: Option<String>,
    /// The user-defined type name.
    pub user_defined_type_name: Option<String>,
    /// The user-defined type name that the current table is a temporary table
    /// for.
    pub is_insertable_into: String,
    /// Whether the table is typed.
    pub is_typed: String,
    /// Whether the table is updatable.
    pub commit_action: Option<String>,
}

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}.{}`", self.table_schema, self.table_name)
    }
}

impl Table {
    #[must_use]
    /// Returns whether the provided column is from the current table.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to check.
    pub fn has_column(&self, column: &Column) -> bool {
        self.table_name == column.table_name
            && self.table_schema == column.table_schema
            && self.table_catalog == column.table_catalog
    }

    #[must_use]
    /// Returns whether the table is a view.
    pub fn is_view(&self) -> bool {
        self.table_type == "VIEW"
    }

    #[must_use]
    /// Returns whether the table is temporary.
    pub fn is_temporary(&self) -> bool {
        self.table_type == "LOCAL TEMPORARY" || self.table_type == "GLOBAL TEMPORARY"
    }

    /// Returns the foreign keys of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns representing the foreign keys of the table.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, crate::error::Error> {
        cached_queries::foreign_keys(self, conn)
    }

    /// Returns the set of foreign tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A set of tables that are foreign to the current table.
    ///
    ///
    /// # Errors
    ///
    /// * If the foreign tables cannot be loaded from the database.
    pub fn foreign_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, crate::error::Error> {
        let mut tables = Vec::new();
        for foreign_key_constraint in self.foreign_keys(conn)? {
            tables.push(foreign_key_constraint.foreign_table(conn)?);
        }
        tables.sort_unstable();
        tables.dedup();
        Ok(tables)
    }

    /// Returns the indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn indices(&self, conn: &mut PgConnection) -> Result<Vec<PgIndex>, crate::error::Error> {
        cached_queries::indices(self, conn)
    }

    /// Returns the UNIQUE constraint indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn unique_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, crate::error::Error> {
        cached_queries::unique_indices(self, conn)
    }

    /// Returns all tables in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_catalog` - The table catalog.
    /// * `table_schema` - The table schema.
    ///
    /// # Returns
    ///
    /// A vector of all tables in the database.
    ///
    /// # Errors
    ///
    /// * If the tables cannot be loaded from the database.
    pub fn load_all(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: &str,
    ) -> Result<Vec<Self>, crate::error::Error> {
        cached_queries::load_all_tables(table_catalog, table_schema, conn)
    }

    /// Returns the table by name.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_name` - The name of the table.
    /// * `table_schema` - The schema of the table.
    /// * `table_catalog` - The catalog of the table.
    ///
    /// # Returns
    ///
    /// The table.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn load(
        table_name: &str,
        table_schema: &str,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Self, crate::error::Error> {
        cached_queries::load_table(conn, table_name, table_schema, table_catalog)
    }

    /// Returns the columns of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the columns cannot be loaded from the database.
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, crate::error::Error> {
        cached_queries::columns(self, conn)
    }

    /// Returns the column by name.
    ///
    /// # Arguments
    ///
    /// * `column_name` - The name of the column.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The column.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    pub fn column_by_name(
        &self,
        column_name: &str,
        conn: &mut PgConnection,
    ) -> Result<Column, crate::error::Error> {
        cached_queries::column_by_name(self, column_name, conn)
    }

    /// Returns whether the table has primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has primary keys.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn has_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, crate::error::Error> {
        self.primary_key_columns(conn).map(|columns| !columns.is_empty())
    }

    /// Returns whether the table has columns that are NOT primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the provided database connection is invalid.
    pub fn has_non_primary_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        self.non_primary_key_columns(conn).map(|columns| !columns.is_empty())
    }

    /// Returns the columns NOT composing the primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the connection to the database fails.
    pub fn non_primary_key_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, crate::error::Error> {
        let mut columns = self.columns(conn)?;
        let primary_key_columns = self.primary_key_columns(conn)?;
        columns.retain(|column| !primary_key_columns.contains(column));
        Ok(columns)
    }

    /// Returns the columns composing the primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, crate::error::Error> {
        cached_queries::primary_key_columns(self, conn)
    }

    /// Returns whether the table has a composite primary key.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has a composite primary key.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn has_composite_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, crate::error::Error> {
        Ok(self.primary_key_columns(conn)?.len() > 1)
    }

    /// Returns the check constraints for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of check constraints.
    ///
    /// # Errors
    ///
    /// * If the check constraints cannot be loaded from the database.
    pub fn check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<CheckConstraint>, crate::error::Error> {
        cached_queries::check_constraints(self, conn)
    }

    /// Returns the list of Triggers associates to the current table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of triggers.
    ///
    /// # Errors
    ///
    /// * If the triggers cannot be loaded from the database.
    pub fn triggers(&self, conn: &mut PgConnection) -> Result<Vec<PgTrigger>, crate::error::Error> {
        cached_queries::triggers(self, conn)
    }
}

impl AsRef<Table> for Table {
    fn as_ref(&self) -> &Table {
        self
    }
}
