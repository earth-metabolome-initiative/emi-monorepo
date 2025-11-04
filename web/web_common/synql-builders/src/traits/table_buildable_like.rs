//! Submodule providing the `TableBuildable` trait for SynQL table buildables.

use synql_core::{
    structs::{InternalDataRef, Workspace},
    traits::TableSynLike,
};
use synql_diesel_schema::traits::TableSchema;

use crate::structs::TableBuildable;

/// Name of the module containing the buildable for a table.
pub const BUILDABLE_MODULE_NAME: &str = "buildable";

/// Trait representing a SynQL table buildable.
pub trait TableBuildableLike: TableSchema {
    /// Returns the crate name which contains the buildable struct, and
    /// associated traits.
    fn table_buildable_crate_name(&self) -> String {
        format!("{}_builders", self.table_singular_snake_name())
    }

    /// Returns the name of the buildable struct for the table.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use synql_settable::prelude::*;
    /// let db = ParserDB::try_from("CREATE TABLE my_table (id INT PRIMARY KEY);")?;
    /// let table = db.table(None, "my_table").unwrap();
    /// assert_eq!(table.table_buildable_name(), "MyTableBuilder");
    /// # Ok(())
    /// # }
    /// ```
    fn table_buildable_name(&self) -> String {
        format!("{}Builder", self.table_singular_camel_name())
    }

    /// Returns the [`TableBuildable<'data, 'table,
    /// Self>`](crate::structs::TableBuildable) representing the buildable
    /// for the table.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    fn buildable<'table, 'data>(
        &'table self,
        workspace: &'table Workspace<'data>,
        database: &'table Self::DB,
    ) -> TableBuildable<'data, 'table, Self>
    where
        Self: 'data,
    {
        TableBuildable::new(self, workspace, database)
    }
}
