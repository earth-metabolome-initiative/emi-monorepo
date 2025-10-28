//! Submodule providing the `TableInsertable` trait for SynQL table insertables.

use synql_core::{
    structs::{InternalDataRef, Workspace},
    traits::TableSynLike,
};
use synql_diesel_schema::traits::TableSchema;

use crate::structs::TableInsertable;

/// Name of the module containing the insertable for a table.
pub const INSERTABLE_MODULE_NAME: &str = "insertable";

/// Trait representing a SynQL table insertable.
pub trait TableInsertableLike: TableSchema {
    /// Returns the name of the crate which will contain the diesel insertable
    /// for the table.
    fn table_insertable_crate_name(&self) -> String {
        format!("{}_insertable", self.table_singular_snake_name())
    }

    /// Returns the name of the insertable struct for the table.
    fn table_insertable_name(&self) -> String {
        format!("New{}", self.table_singular_camel_name())
    }

    /// Returns the
    /// [`TableInsertable<'data, 'table,
    /// Self>`](crate::structs::TableInsertable) representing the insertable
    /// for the table.
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

    /// Returns a reference to the insertable module ref for the table.
    fn insertable_ref<'data>(
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

impl<T: TableSynLike> TableInsertableLike for T {}
