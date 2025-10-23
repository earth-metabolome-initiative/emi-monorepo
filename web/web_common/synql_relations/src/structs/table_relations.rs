//! Submodule defining the `TableRelations` struct.

use crate::traits::TableRelationsLike;

#[derive(Debug)]
/// Struct representing a SynQL table relations trait.
pub struct TableRelations<'data, 'table, T: TableRelationsLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::Database,
}

impl<'data, 'table, T: TableRelationsLike + ?Sized> TableRelations<'data, 'table, T> {
    /// Creates a new `TableRelations` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing `TableRelationsLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::Database,
    ) -> Self {
        Self { table, workspace, database }
    }
}
