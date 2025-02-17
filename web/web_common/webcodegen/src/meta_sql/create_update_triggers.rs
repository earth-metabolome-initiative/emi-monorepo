//! Submodule to create triggers to automatically update the `updated_at` column
//! of a table.
//!
//! The core functionality is provided by the creation of the
//! `updated_at_trigger` function, which sets the `updated_at` column to the
//! current timestamp. The metaprogramming here solely checks that the table
//! being parsed indeed has a column named `updated_at`, and that it is of type
//! `timestamp`.
//!
//! ```sql
//! CREATE OR REPLACE FUNCTION updated_at_trigger()
//! RETURNS TRIGGER AS $$
//! BEGIN
//!     NEW.updated_at = NOW();
//!     RETURN NEW;
//! END;
//! $$ LANGUAGE plpgsql;
//! ```
//!
//! Secondarily, for each table that has an `updated_at` column, a trigger is
//! created that calls the `updated_at_trigger` function before an `UPDATE`
//! operation is performed. In the following example, the `users` table has an
//! `updated_at` column, and a trigger is created for it.
//!
//! ```sql
//! CREATE TRIGGER users_updated_at_trigger
//! BEFORE UPDATE ON users
//! FOR EACH ROW
//! EXECUTE FUNCTION updated_at_trigger();
//! ```

use diesel::{connection::SimpleConnection, PgConnection};

use crate::{errors::WebCodeGenError, Table};

impl Table {
    /// Returns the SQL code to create the `updated_at_trigger` function.
    pub fn updated_at_trigger_function() -> &'static str {
        concat!(
            r#"CREATE OR REPLACE FUNCTION updated_at_trigger()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;"#
        )
    }

    /// Returns whether the table has an `updated_at` column.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_updated_at_column(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().any(|column| column.is_updated_at()))
    }

    /// Returns the expected name of the current table trigger.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the table does not have an `updated_at` column.
    /// * Returns an error if the provided database connection is invalid.
    pub fn updated_at_trigger_name(
        &self,
        conn: &mut PgConnection,
    ) -> Result<String, WebCodeGenError> {
        if !self.has_updated_at_column(conn)? {
            return Err(WebCodeGenError::MissingUpdateAtColumn(Box::new(self.clone())));
        }

        Ok(format!("{table_name}_updated_at_trigger", table_name = self.table_name))
    }

    /// Returns whether the current updated_at trigger exists.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the table does not have an `updated_at` column.
    /// * Returns an error if the provided database connection is invalid.
    pub fn updated_at_trigger_exists(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let trigger_name = self.updated_at_trigger_name(conn)?;

        Ok(self.triggers(conn)?.iter().any(|trigger| trigger.tgname == trigger_name))
    }

    /// Returns the SQL code to create the trigger for the `updated_at` column.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the table does not have an `updated_at` column.
    /// * Returns an error if the provided database connection is invalid.
    pub fn updated_at_trigger(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        if !self.has_updated_at_column(conn)? {
            return Err(WebCodeGenError::MissingUpdateAtColumn(Box::new(self.clone())));
        }

        Ok(format!(
            r#"CREATE OR REPLACE TRIGGER {table_name}_updated_at_trigger
BEFORE UPDATE ON {table_name}
FOR EACH ROW
EXECUTE FUNCTION updated_at_trigger();"#,
            table_name = self.table_name
        ))
    }

    /// Returns the SQL code to create the trigger for the `updated_at` column
    /// for all tables with an `updated_at` column that do not already have
    /// the trigger.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_catalog` - The name of the database.
    /// * `table_schema` - The name of the schema.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn create_update_triggers(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<(), WebCodeGenError> {
        conn.batch_execute(Self::updated_at_trigger_function())?;
        for table in Table::load_all(conn, table_catalog, table_schema)? {
            if table.has_updated_at_column(conn)? && !table.updated_at_trigger_exists(conn)? {
                let trigger = table.updated_at_trigger(conn)?;
                conn.batch_execute(&trigger)?;
            }
        }
        Ok(())
    }
}
