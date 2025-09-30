//! Submodule defining the `Authored` trait which provides author-related
//! columns.

use diesel::PgConnection;
use pg_diesel::models::{Column, Table};

use crate::traits::Extension;

/// Trait characterizing whether a table has an author column,
/// i.e. a column indicating who created a row in this table,
/// and related columns.
pub trait Authored: Extension {
    /// Returns the column which indicates who created a row in this table, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `users` - The users table.
    /// * `recursive` - Whether to search ancestor tables for a `created_by`
    ///   column.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    fn created_by(
        &self,
        users: &Table,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error>;

    /// Returns the column which indicates who updated a row in this table, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `users` - The users table.
    /// * `recursive` - Whether to search ancestor tables for a `updated_by`
    ///   column.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    fn updated_by(
        &self,
        users: &Table,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error>;
}

fn user_column(
    table: &Table,
    users: &Table,
    recursive: bool,
    column_name: &str,
    conn: &mut PgConnection,
) -> Result<Option<Column>, diesel::result::Error> {
    for column in table.columns(conn)? {
        if column.column_name != column_name {
            continue;
        }
        for foreign_key in column.foreign_keys(conn)? {
            if foreign_key.foreign_table(conn)? == *users
                && foreign_key.is_foreign_primary_key(conn)?
            {
                return Ok(Some(column));
            }
        }
    }

    if recursive {
        for extended_table in table.extended_tables(conn)? {
            if let Some(column) = extended_table.created_by(users, true, conn)? {
                return Ok(Some(column));
            }
        }
    }

    Ok(None)
}

impl Authored for Table {
    fn created_by(
        &self,
        users: &Table,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error> {
        user_column(self, users, recursive, "created_by", conn)
    }

    fn updated_by(
        &self,
        users: &Table,
        recursive: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, diesel::result::Error> {
        user_column(self, users, recursive, "updated_by", conn)
    }
}
