//! Submodule defining the `TableBuildable` struct for generating settable
//! traits.

mod into_crate;
mod into_data;
mod into_data_variant;
mod into_module;
mod into_struct;
mod key_settable;
mod value_settable;

use synql_core::structs::Workspace;

use crate::traits::TableBuildableLike;

#[derive(Debug)]
/// Struct representing a SynQL table buildable struct.
pub struct TableBuildable<'table, T: TableBuildableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableBuildableLike + ?Sized> Clone for TableBuildable<'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableBuildableLike + ?Sized> Copy for TableBuildable<'table, T> {}

impl<'table, T: TableBuildableLike + ?Sized> TableBuildable<'table, T> {
    /// Creates a new [`TableBuildable`](crate::structs::TableBuildable)
    /// representing the buildable for the table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model to create the buildable for.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    #[inline]
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }
}
