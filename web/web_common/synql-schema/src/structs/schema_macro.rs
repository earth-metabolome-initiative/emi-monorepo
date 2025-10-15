//! Submodule defining the `SchemaMacro` struct to represent an SQL schema which
//! can be printed out in the context of a `quote` macro.

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use synql_core::prelude::ColumnLike;
use synql_core::{
    structs::Workspace,
    traits::{ColumnSynLike, TableSynLike},
};

/// Struct representing a diesel schema.
pub struct SchemaMacro<'table, T: TableSynLike> {
    /// The table represented by the schema.
    table: &'table T,
    /// The workspace where the table is defined.
    workspace: &'table Workspace,
    /// The database connection to use to query the table schema.
    database: &'table T::Database,
}

impl<'table, T: TableSynLike> SchemaMacro<'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table Workspace,
        database: &'table T::Database,
    ) -> Self {
        Self { table, workspace, database }
    }
}

impl<T> ToTokens for SchemaMacro<'_, T>
where
    T: TableSynLike,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut columns = Vec::new();
        for column in self.table.columns(self.database) {
            let column_attribute = column.column_snake_ident();
            let column_type = column.diesel_type(self.workspace, self.database).unwrap_or_else(|| {
                panic!(
                    "Could not find diesel type {} for column '{}' of table '{}'",
                    column.normalized_data_type(),
                    column.column_name(),
                    self.table.table_name()
                )
            });
            columns.push(if column.has_snake_case_column_name() {
                quote! {
                    #column_attribute -> #column_type
                }
            } else {
                let original_column_name = column.column_name();
                quote! {
                    #[sql_name = #original_column_name]
                    #column_attribute -> #column_type
                }
            });
        }
        let primary_key = if self.table.has_primary_key(self.database) {
            let primary_keys = self.table.primary_key_idents(self.database);
            Some(quote! {
                (#(#primary_keys),*)
            })
        } else {
            None
        };

        let sql_name = if self.table.has_snake_case_table_name() {
            None
        } else {
            let original_table_name = self.table.table_name();
            Some(quote! {#[sql_name = #original_table_name]})
        };

        let table_name_ident = self.table.table_snake_ident();

        tokens.extend(quote! {
            diesel::table! {
                #sql_name
                #table_name_ident #primary_key {
                    #(#columns),*
                }
            }
        })
    }
}
