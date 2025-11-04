//! Submodule providing the `TableInsertable` trait for SynQL table insertables.

use sql_traits::traits::{ColumnLike, DatabaseLike};
use synql_core::structs::{InternalDataRef, Workspace};
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

    /// Iterates the insertable columns for the table.
    ///
    /// # Arguments
    ///
    /// * `database` - The database connection to use to query the table
    ///   columns.
    fn insertable_columns<'table>(
        &'table self,
        database: &'table Self::DB,
    ) -> impl Iterator<Item = &'table <Self::DB as DatabaseLike>::Column> + 'table {
        self.columns(database).filter(move |column| !column.is_generated())
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

    /// Returns the reference to the [`InternalDataRef`] for the table
    /// insertable.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    fn insertable_data_ref<'data>(
        &self,
        workspace: &Workspace<'data>,
    ) -> Option<InternalDataRef<'data>> {
        let crate_ref = workspace.internal_crate(&self.table_insertable_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_insertable_name())?,
        ))
    }
}

impl<T: TableSchema> TableInsertableLike for T {}
