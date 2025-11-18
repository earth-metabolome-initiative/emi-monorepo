//! Submodule defining the `TableInsertable` struct.

use quote::quote;
use sql_traits::traits::{ColumnLike, TableLike};
use synql_core::{
    prelude::Builder,
    structs::{Decorator, Derive, InternalToken},
};

use crate::traits::TableInsertableLike;

mod into_crate;
mod into_data;
mod into_module;
mod set_column;

#[derive(Debug)]
/// Struct representing a SynQL table insertable.
pub struct TableInsertable<'table, T: TableLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableLike + ?Sized> Clone for TableInsertable<'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableLike + ?Sized> Copy for TableInsertable<'table, T> {}

impl<'table, T: TableInsertableLike + ?Sized> TableInsertable<'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    /// Returns the derives for the insertable struct.
    fn diesel_derives(&self) -> Derive {
        let traits = vec![
            self.workspace.external_trait("Insertable").expect("Failed to get Insertable trait"),
        ];

        Derive::new().add_traits(traits).build().expect("Failed to build Insertable derive")
    }

    /// Returns the table decorator for the insertable struct.
    fn table_decorator(&self) -> Decorator {
        let table_schema = self
            .table
            .schema_module(self.workspace)
            .expect("Failed to get the table schema module");
        let snake_case_ident = self.table.table_snake_ident();

        Decorator::new()
            .token(
                InternalToken::new()
                    .private()
                    .stream(quote! {
                        diesel(table_name = #table_schema::#snake_case_ident)
                    })
                    .internal_module(table_schema)
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .build()
            .unwrap()
    }

    /// Generates implementations of the `MaybeGetColumn` trait for all
    /// insertable columns in the table.
    fn maybe_get_column_impls(&self) -> Vec<InternalToken> {
        use synql_core::traits::ColumnSynLike;
        use synql_diesel_schema::traits::ColumnSchema;

        let mut maybe_get_column_impls = Vec::new();
        let table_insertable_name = self.table.table_insertable_name();
        let table_insertable: syn::Ident = syn::parse_str(&table_insertable_name).unwrap();

        let maybe_get_column = self
            .workspace
            .external_trait("MaybeGetColumn")
            .expect("Failed to get MaybeGetColumn trait");

        for column in self.table.insertable_columns(self.database) {
            let column_path = column.column_path(self.database);
            let column_type = column.rust_type(self.workspace, self.database).unwrap();
            let column_ident = column.column_snake_ident();

            let column_getter = if column.is_nullable(self.database) {
                quote! { Some(&self.#column_ident) }
            } else {
                quote! { self.#column_ident.as_ref() }
            };

            let maybe_get_column_impl = InternalToken::new()
                .private()
                .stream(quote! {
                    impl #maybe_get_column<#column_path> for #table_insertable {
                        fn maybe_get_column(&self) -> Option<&#column_type> {
                            #column_getter
                        }
                    }
                })
                .implemented_trait(maybe_get_column.clone().into())
                .build()
                .unwrap();

            maybe_get_column_impls.push(maybe_get_column_impl);
        }

        maybe_get_column_impls
    }
}
