//! Submodule defining the `TableTransitiveExtension` struct.

use synql_core::structs::Workspace;

use crate::traits::TableTransitiveExtensionLike;

mod into_crate;
mod into_module;
mod into_trait;

#[derive(Debug)]
/// Struct representing a SynQL table transitive extension trait.
pub struct TableTransitiveExtension<'table, T: TableTransitiveExtensionLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableTransitiveExtensionLike + ?Sized> Clone
    for TableTransitiveExtension<'table, T>
{
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableTransitiveExtensionLike + ?Sized> Copy
    for TableTransitiveExtension<'table, T>
{
}

impl<'table, T: TableTransitiveExtensionLike + ?Sized> TableTransitiveExtension<'table, T> {
    /// Creates a new `TableTransitiveExtension` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing `TableTransitiveExtensionLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }
}
