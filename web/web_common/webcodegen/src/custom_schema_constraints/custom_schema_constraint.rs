use std::path::Path;

use diesel::PgConnection;

use crate::{Column, Table, errors::WebCodeGenError};

/// A trait for custom table constraints
pub trait CustomTableConstraint {
    /// The error type for the constraint
    type Error: From<diesel::result::Error>
        + From<std::io::Error>
        + std::fmt::Display
        + From<WebCodeGenError>;

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
        // If there exists in the current directory a file named following the pattern
        // `{constraint_name}_{table_catalog}_{table_schema}_{table_name}.err` where
        // `constraint_name` is the name of the constraint (i.e., the name of
        // the struct implementing this trait), `table_catalog` is the catalog
        // name, `table_schema` is the schema name, and `table_name` is the name of the
        // table that violated the constraint, then we identify the name of that
        // table and start the checking from that table, and after checking that
        // table, we remove that file and resume checking the rest of the tables.

        let constraint_name = std::any::type_name::<Self>();
        let file_name_prefix = format!("{constraint_name}_{table_catalog}_{table_schema}_");
        let mut start_table: Option<String> = None;
        for entry in std::fs::read_dir(".")?.flatten() {
            if let Ok(file_name) = entry.file_name().into_string()
                && file_name.starts_with(&file_name_prefix)
                && Path::new(&file_name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("err"))
            {
                let table_name = file_name
                    .trim_start_matches(&file_name_prefix)
                    .trim_end_matches(".err")
                    .to_string();
                start_table = Some(table_name);
                std::fs::remove_file(entry.path())?;
                break;
            }
        }

        if let Some(table_name) = start_table {
            let table = Table::load(conn, &table_name, table_schema, table_catalog)?;
            if let Err(err) = self.check_constraint(conn, &table) {
                // We store this error into a file.
                let file_name = format!("{file_name_prefix}{}.err", table.table_name);
                std::fs::write(&file_name, err.to_string())?;

                return Err(err);
            }
        }

        for table in Table::load_all(conn, table_catalog, table_schema)?.as_ref() {
            if let Err(err) = self.check_constraint(conn, table) {
                // We store this error into a file.
                let file_name = format!("{file_name_prefix}{}.err", table.table_name);
                std::fs::write(&file_name, err.to_string())?;

                return Err(err);
            }
        }
        Ok(())
    }
}

/// A trait for custom column constraints
pub trait CustomColumnConstraint {
    /// The error type for the constraint
    type Error: From<diesel::result::Error>
        + From<std::io::Error>
        + std::fmt::Display
        + From<WebCodeGenError>;

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
        for table in Table::load_all(conn, table_catalog, table_schema)?.as_ref() {
            for column in table.columns(conn)?.as_ref() {
                self.check_constraint(conn, column)?;
            }
        }
        Ok(())
    }
}
