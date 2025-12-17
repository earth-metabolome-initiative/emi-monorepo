//! Submodule providing the `TableInsertable` trait for SynQL table insertables.

use sql_traits::traits::{ColumnLike, DatabaseLike};
use synql_core::structs::{InternalDataRef, Workspace};
use synql_models::traits::TableModelLike;

use crate::structs::TableInsertable;

/// Name of the module containing the insertable for a table.
pub const INSERTABLE_MODULE_NAME: &str = "insertable";

/// Trait representing a SynQL table insertable.
pub trait TableInsertableLike: TableModelLike {
    /// Returns the name of the crate for the table insertable.
    fn table_insertable_crate_name(&self) -> String {
        format!("{}_insertable", self.table_singular_snake_name())
    }

    /// Returns the name of the insertable struct for the table.
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
    fn insertable<'table>(
        &'table self,
        workspace: &'table Workspace,
        database: &'table Self::DB,
    ) -> TableInsertable<'table, Self> {
        TableInsertable::new(self, workspace, database)
    }

    /// Returns the reference to the [`InternalDataRef`] for the table
    /// insertable.
    ///
    /// # Arguments
    ///
    /// * `workspace` - The workspace where the table is defined.
    fn insertable_data_ref(&self, workspace: &Workspace) -> Option<InternalDataRef> {
        let crate_ref = workspace.internal_crate(&self.table_insertable_crate_name())?;
        Some(InternalDataRef::new(
            crate_ref,
            crate_ref.internal_data(&self.table_insertable_name())?,
        ))
    }
}

impl<T: TableModelLike> TableInsertableLike for T {}
