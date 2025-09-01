use diesel::PgConnection;

use crate::{Column, Table};

/// A trait for custom table constraints
pub trait CustomTableConstraint {
    /// The error type for the constraint
    type Error: From<diesel::result::Error>;

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
    fn check_constraint(&self, conn: &mut PgConnection, table: &Table) -> Result<(), Self::Error>;

    /// Check the table constraint on all tables in the database
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the catalog to filter the tables by
    /// * `table_schema` - The name of the schema to filter the tables by
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If loading the tables from the database fails
    /// * If checking the constraint fails
    fn check_all(
        &self,
        table_catalog: &str,
        table_schema: &str,
        conn: &mut PgConnection,
    ) -> Result<(), Self::Error> {
        for table in Table::load_all(conn, table_catalog, table_schema)? {
            self.check_constraint(conn, &table)?;
        }
        Ok(())
    }
}

/// A trait for custom column constraints
pub trait CustomColumnConstraint {
    /// The error type for the constraint
    type Error: From<diesel::result::Error>;

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
    fn check_constraint(&self, conn: &mut PgConnection, column: &Column)
    -> Result<(), Self::Error>;

    /// Runs the check on all of the columns in the database.
    ///
    /// # Arguments
    ///
    /// * `table_catalog` - The name of the catalog to filter the columns by
    /// * `table_schema` - The name of the schema to filter the columns by
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
        table_schema: &str,
        conn: &mut PgConnection,
    ) -> Result<(), Self::Error> {
        for table in Table::load_all(conn, table_catalog, table_schema)? {
            for column in table.columns(conn)? {
                self.check_constraint(conn, &column)?;
            }
        }
        Ok(())
    }
}
