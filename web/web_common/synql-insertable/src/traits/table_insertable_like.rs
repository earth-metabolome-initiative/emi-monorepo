//! Submodule providing the `TableInsertable` trait for SynQL table insertables.

use synql_core::structs::Workspace;
use synql_diesel_schema::traits::TableSchema;
use synql_value_settable::traits::TableValueSettableLike;

use crate::structs::TableInsertable;

/// Name of the module containing the insertable for a table.
pub const INSERTABLE_MODULE_NAME: &str = "insertable";

/// Trait representing a SynQL table insertable.
pub trait TableInsertableLike: TableValueSettableLike {
    /// Returns the name of the crate for the table insertable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE users (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "users").unwrap();
    /// assert_eq!(table.table_insertable_crate_name(), "user_insertable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_insertable_crate_name(&self) -> String {
        format!("{}_insertable", self.table_singular_snake_name())
    }

    /// Returns the name of the insertable struct for the table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_insertable_name(), "NewMyTable");
    /// # Ok(())
    /// # }
    /// ```
    fn table_insertable_name(&self) -> String {
        format!("New{}", self.table_singular_camel_name())
    }

    /// Returns the [`TableInsertable`] representing the insertable for the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   insertable.
    fn insertable<'table, 'data>(
        &'table self,
        workspace: &'table Workspace<'data>,
        database: &'table Self::DB,
    ) -> TableInsertable<'data, 'table, Self>
    where
        Self: 'data,
    {
        TableInsertable::new(self, workspace, database)
    }
}

impl<T: TableSchema> TableInsertableLike for T {}
