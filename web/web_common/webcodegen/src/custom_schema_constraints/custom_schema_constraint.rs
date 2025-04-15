use diesel::pg::PgConnection;

use crate::{Column, Table, errors::WebCodeGenError};

/// A trait for custom table constraints
pub trait CustomTableConstraint {
    /// Check the table constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `table` - A reference to a `Table`
    ///
    /// # Errors
    ///
    /// * If the constraint check fails
    /// * If the database query fails
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError>;

    /// Check the table constraint on all tables in the database
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the catalog to filter the tables by
    /// * `table_schema` - An optional schema name to filter the tables by
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If loading the tables from the database fails
    /// * If checking the constraint fails
    fn check_all(
        &self,
        table_catalog: &str,
        table_schema: Option<&str>,
        conn: &mut PgConnection,
    ) -> Result<(), WebCodeGenError> {
        for table in Table::load_all(conn, table_catalog, table_schema)? {
            self.check_constraint(conn, &table)?;
        }
        Ok(())
    }
}

/// A trait for custom column constraints
pub trait CustomColumnConstraint {
    /// Check the column constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    /// * `column` - A reference to a `Column`
    ///
    /// # Errors
    ///
    /// * If the constraint check fails
    /// * If the database query fails
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError>;

    /// Runs the check on all of the columns in the database.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the catalog to filter the columns by
    /// * `table_schema` - An optional schema name to filter the columns by
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If loading the tables from the database fails
    /// * If loading the columns from the database fails
    /// * If checking the constraint fails
    fn check_all(
        &self,
        table_catalog: &str,
        table_schema: Option<&str>,
        conn: &mut PgConnection,
    ) -> Result<(), WebCodeGenError> {
        for table in Table::load_all(conn, table_catalog, table_schema)? {
            for column in table.columns(conn)? {
                self.check_constraint(conn, &column)?;
            }
        }
        Ok(())
    }
}
