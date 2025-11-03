//! Submodule defining the `TableBuildable` struct for generating settable
//! traits.

mod into_crate;

use quote::{ToTokens, quote};
use sql_relations::traits::TriangularSameAsForeignKeyLike;
use sql_traits::traits::{ColumnLike, DatabaseLike, ForeignKeyLike};
use synql_core::{
    prelude::Builder,
    structs::{
        Argument, DataVariantRef, Documentation, InternalToken, InternalTrait, Method,
        MethodBuilder, WhereClause, Workspace,
    },
    traits::{ColumnSynLike, TableSynLike},
};

use crate::traits::TableBuildableLike;

#[derive(Debug)]
/// Struct representing a SynQL table buildable struct.
pub struct TableBuildable<'data, 'table, T: TableBuildableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableBuildableLike + ?Sized> Clone for TableBuildable<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableBuildableLike + ?Sized> Copy for TableBuildable<'data, 'table, T> {}

impl<'data, 'table, T: TableBuildableLike + ?Sized> TableBuildable<'data, 'table, T> {
    /// Creates a new [`TableBuildable<'data, 'table,
    /// T>`](crate::structs::TableBuildable) representing the buildable for
    /// the table.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model to create the buildable for.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table
    ///   buildable.
    pub fn new(
        table: &'table T,
        workspace: &'table Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }
}
