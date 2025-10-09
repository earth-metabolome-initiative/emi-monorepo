//! Submodule defining the `MostConcrete` trait which provides method relative
//! to dispatching the most-concrete table in an inheritance hierarchy.

use diesel::PgConnection;
use pg_diesel::models::{Column, Table};

use crate::traits::Extension;

/// Trait characterizing whether a table has the `most_concrete_table` column,
/// i.e. a column indicating the most concrete table in an inheritance
/// hierarchy.
pub trait MostConcrete: Extension {
    /// Returns the column which indicates the most concrete table in an
    /// inheritance hierarchy, if any.
    ///
    /// # Arguments
    ///
    /// * `recursive` - Whether to search ancestor tables for a
    ///   `most_concrete_table` column.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    fn most_concrete_table(
        &self,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error>;
}

impl MostConcrete for Table {
    fn most_concrete_table(
        &self,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error> {
        for column in self.columns(conn)? {
            if column.column_name == "most_concrete_table" {
                return Ok(Some(column));
            }
        }
        if recursive {
            for extended_table in self.extended_tables(conn)? {
                if let Some(column) = extended_table.most_concrete_table(true, conn)? {
                    return Ok(Some(column));
                }
            }
        }
        Ok(None)
    }
}
