//! Submodule defining the `Authored` trait which provides author-related
//! columns.

use sql_traits::traits::{DatabaseLike, TableLike};

/// Trait defining a database which contains a users table.
pub trait AuthoredDatabaseLike: DatabaseLike {
    /// Returns the users table.
    fn users_table(&self) -> &Self::Table;

    /// Returns the expected name of the column indicating who created
    /// a row in a table.
    fn created_by_column_name(&self) -> &str;

    /// Returns the expected name of the column indicating who last
    /// updated a row in a table.
    fn updated_by_column_name(&self) -> &str;
}

/// Trait characterizing whether a table has an author column,
/// i.e. a column indicating who created a row in this table,
/// and related columns.
pub trait AuthoredTableLike:
    TableLike<Database = <Self as AuthoredTableLike>::AuthoredDatabase>
{
    /// Type of the database the table belongs to.
    type AuthoredDatabase: AuthoredDatabaseLike<Table = Self, Column = Self::Column>;

    /// Returns the column which indicates who created a row in this table, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `database` - The database the table belongs to.
    /// * `recursive` - Whether to search ancestor tables for a `created_by`
    ///   column.
    fn created_by<'db>(
        &'db self,
        database: &'db Self::AuthoredDatabase,
        recursive: bool,
    ) -> Option<&'db Self::Column>
    where
        Self: 'db,
    {
        if let Some(column) = self.column_by_name(database.created_by_column_name(), database) {
            // We also check that the column actually references the users table.
            if self.referenced_tables_via_column(database, column).contains(&database.users_table())
            {
                return Some(column);
            }
        };
        if recursive {
            for parent in self.extended_tables(database) {
                if let Some(col) = parent.created_by(database, true) {
                    return Some(col);
                }
            }
        }
        None
    }

    /// Returns the column which indicates who updated a row in this table, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `database` - The database the table belongs to.
    /// * `recursive` - Whether to search ancestor tables for a `updated_by`
    ///   column.
    fn updated_by<'db>(
        &'db self,
        database: &'db Self::AuthoredDatabase,
        recursive: bool,
    ) -> Option<&'db Self::Column>
    where
        Self: 'db,
    {
        if let Some(column) = self.column_by_name(database.updated_by_column_name(), database) {
            // We also check that the column actually references the users table.
            if self.referenced_tables_via_column(database, column).contains(&database.users_table())
            {
                return Some(column);
            }
        };
        if recursive {
            for parent in self.extended_tables(database) {
                if let Some(col) = parent.updated_by(database, true) {
                    return Some(col);
                }
            }
        }
        None
    }
}
