//! Submodule defining the Extension trait which characterizes whether a table
//! is an extension of another table.

use diesel::PgConnection;
use pg_diesel::models::{KeyColumnUsage, Table};

use crate::functions::is_extension_foreign_key;

/// Trait characterizing whether a table is an extension of another table,
/// i.e. it has the same primary key as another table and possibly additional
/// columns, which is defined by a primary key which also is a foreign key to
/// another table's primary key.
pub trait Extension {
    /// Returns the foreign key constraint which defines the extension
    /// relationship, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn extension_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, diesel::result::Error>;

    /// Returns the tables that this table is extending.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn extended_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, diesel::result::Error> {
        let mut extended_tables = Vec::new();
        for foreign_key in self.extension_foreign_keys(conn)? {
            extended_tables.push(foreign_key.foreign_table(conn)?);
        }
        Ok(extended_tables)
    }

    /// Returns the ancestor tables that this table is extending, recursively.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    /// * If the foreign key constraint cannot be loaded from the database.
    fn ancestor_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, diesel::result::Error> {
        let mut ancestors = Vec::new();
        for extended_table in self.extended_tables(conn)? {
            ancestors.extend(extended_table.ancestor_tables(conn)?);
            ancestors.push(extended_table);
        }
        Ok(ancestors)
    }

    /// Returns whether the table is extending some other table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn is_extension(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        Ok(!self.extension_foreign_keys(conn)?.is_empty())
    }

    /// Returns whether the table is an extension of the given table.
    ///
    /// # Arguments
    ///
    /// * `other` - The other table to check against.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn is_extension_of(
        &self,
        other: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        for foreign_key in self.extension_foreign_keys(conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if foreign_table == *other {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Returns whether the table shares any ancestor with the given table.
    ///
    /// # Arguments
    ///
    /// * `other` - The other table to check against.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign key constraint cannot be loaded from the database.
    fn shares_ancestors_with(
        &self,
        other: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, diesel::result::Error> {
        let self_ancestors = self.ancestor_tables(conn)?;
        let other_ancestors = other.ancestor_tables(conn)?;

        for ancestor in self_ancestors {
            if other_ancestors.contains(&ancestor) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Extension for Table {
    fn extension_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, diesel::result::Error> {
        let mut extension_foreign_keys = Vec::new();
        let foreign_keys = self.foreign_keys(conn)?;
        for foreign_key in foreign_keys.iter() {
            if is_extension_foreign_key(foreign_key, conn)? {
                extension_foreign_keys.push(foreign_key.clone());
            }
        }
        Ok(extension_foreign_keys)
    }
}
