use async_trait::async_trait;
use diesel_async::AsyncPgConnection;

use crate::{Column, Table, errors::WebCodeGenError};

#[async_trait]
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
    async fn check_constraint(
        &self,
        conn: &mut AsyncPgConnection,
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
    async fn check_all(
        &self,
        table_catalog: &str,
        table_schema: Option<&str>,
        conn: &mut AsyncPgConnection,
    ) -> Result<(), WebCodeGenError> {
        for table in Table::load_all(conn, table_catalog, table_schema).await? {
            self.check_constraint(conn, &table).await?;
        }
        Ok(())
    }
}

#[async_trait]
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
    async fn check_constraint(
        &self,
        conn: &mut AsyncPgConnection,
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
    async fn check_all(
        &self,
        table_catalog: &str,
        table_schema: Option<&str>,
        conn: &mut AsyncPgConnection,
    ) -> Result<(), WebCodeGenError> {
        for table in Table::load_all(conn, table_catalog, table_schema).await? {
            for column in table.columns(conn).await? {
                self.check_constraint(conn, &column).await?;
            }
        }
        Ok(())
    }
}
