//! Submodule defining the `SchemaMacro` struct to represent an SQL schema which
//! can be printed out in the context of a `quote` macro.

use quote::ToTokens;
use synql_core::{
    structs::{InternalToken, Workspace},
    traits::TableSynLike,
};

mod into_crate;
mod into_module;
mod into_token;

/// Struct representing a diesel schema.
pub struct SchemaMacro<'table, T: TableSynLike> {
    /// The table represented by the schema.
    pub(crate) table: &'table T,
    /// The workspace where the table is defined.
    pub(crate) workspace: &'table Workspace,
    /// The database connection to use to query the table schema.
    pub(crate) database: &'table T::DB,
}

impl<'table, T: TableSynLike> SchemaMacro<'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table Workspace,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }
}

impl<'table, T: TableSynLike> Clone for SchemaMacro<'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableSynLike> Copy for SchemaMacro<'table, T> {}

impl<'table, T: TableSynLike> ToTokens for SchemaMacro<'table, T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_token: InternalToken = (*self).into();
        internal_token.to_tokens(tokens);
    }
}
