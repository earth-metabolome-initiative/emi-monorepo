//! Submodule defining the `SchemaMacro` struct to represent an SQL schema which
//! can be printed out in the context of a `quote` macro.

use quote::ToTokens;
use synql_core::{
    prelude::Builder,
    structs::{InternalToken, Workspace},
    traits::{ColumnSynLike, TableSynLike},
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

    /// Generates implementations of the `TypedColumn` trait for all columns in
    /// the table.
    fn typed_column_impls(&self) -> Vec<InternalToken> {
        use quote::quote;

        let mut typed_column_impls = Vec::new();

        let Some(typed_column_trait) = self.workspace.external_trait("TypedColumn") else {
            return typed_column_impls;
        };

        let table_ident = self.table.table_snake_ident();

        for column in self.table.columns(self.database) {
            let column_snake_ident = column.column_snake_ident();
            let column_path: syn::Path = syn::parse_quote!(#table_ident::#column_snake_ident);
            let Some(column_type) = column.rust_type(self.workspace, self.database) else {
                continue;
            };

            let typed_column_impl = InternalToken::new()
                .private()
                .stream(quote! {
                    impl #typed_column_trait for #column_path {
                        type Type = #column_type;
                    }
                })
                .implemented_trait(typed_column_trait.clone().into())
                .build()
                .unwrap();

            typed_column_impls.push(typed_column_impl);
        }

        typed_column_impls
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
