//! Submodule defining the `TableInsertableKeySettable` struct for generating
//! settable traits.

use quote::quote;
use sql_relations::{prelude::ColumnLike, traits::InheritableDatabaseLike};
use sql_traits::traits::DatabaseLike;
use syn::Ident;
use synql_attributes::traits::TableAttributesLike;
use synql_core::{
    prelude::Builder,
    structs::{
        Argument, DataVariantRef, Documentation, InternalToken, Method, WhereClause, Workspace,
    },
    traits::ColumnSynLike,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::traits::TableInsertableKeySettableLike;

mod into_crate;
mod into_module;
mod into_trait;

#[derive(Debug)]
/// Struct representing a SynQL table insertable key settable trait.
pub struct TableInsertableKeySettable<'table, T: TableInsertableKeySettableLike + ?Sized> {
    table: &'table T,
    workspace: &'table Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> Clone
    for TableInsertableKeySettable<'table, T>
{
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> Copy
    for TableInsertableKeySettable<'table, T>
{
}

impl<'table, T: TableInsertableKeySettableLike + ?Sized> TableInsertableKeySettable<'table, T> {
    /// Creates a new `TableInsertableKeySettable` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing
    ///   `TableInsertableKeySettableLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(table: &'table T, workspace: &'table Workspace, database: &'table T::DB) -> Self {
        Self { table, workspace, database }
    }
}
