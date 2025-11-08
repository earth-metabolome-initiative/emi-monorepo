//! Submodule defining the `SchemaMacro` struct to represent an SQL schema which
//! can be printed out in the context of a `quote` macro.

use synql_core::{structs::Workspace, traits::TableSynLike};

mod into_crate;
mod into_module;
mod into_token;

/// Struct representing a diesel schema.
pub struct SchemaMacro<'data, 'table, T: TableSynLike> {
    /// The table represented by the schema.
    pub(crate) table: &'table T,
    /// The workspace where the table is defined.
    pub(crate) workspace: &'table Workspace<'data>,
    /// The database connection to use to query the table schema.
    pub(crate) database: &'table T::DB,
}

impl<'data, 'table, T: TableSynLike> SchemaMacro<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }
}
