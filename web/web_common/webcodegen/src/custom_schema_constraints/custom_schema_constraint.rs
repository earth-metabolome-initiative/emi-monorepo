use crate::errors::WebCodeGenError;
use crate::Column;
use crate::Table;
use diesel::pg::PgConnection;

/// A trait for custom table constraints
pub trait CustomTableConstraint {
    /// Check the table constraint
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        table: &Table,
    ) -> Result<(), WebCodeGenError>;

    /// Check the table constraint on all tables in the database
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
    fn check_constraint(
        &self,
        conn: &mut PgConnection,
        column: &Column,
    ) -> Result<(), WebCodeGenError>;

    /// Runs the check on all of the columns in the database.
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
