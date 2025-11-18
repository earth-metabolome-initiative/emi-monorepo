//! Submodule implementing the generation of the `MaybeGetColumn` trait for the
//! `TableBuildable` struct.

use quote::quote;
use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{prelude::Builder, structs::InternalToken, traits::ColumnSynLike};
use synql_diesel_schema::traits::ColumnSchema;

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> TableBuildable<'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Generates implementations of the `MaybeGetColumn` trait for all
    /// columns in the table's builder.
    pub(crate) fn maybe_get_column_impls(&self) -> Vec<InternalToken> {
        let mut maybe_get_column_impls = Vec::new();
        let builder_ident = self.table.table_buildable_ident();

        let maybe_get_column = self
            .workspace
            .external_trait("MaybeGetColumn")
            .expect("Failed to get MaybeGetColumn trait");

        let insertable_ident = self.table.table_singular_snake_ident();
        let insertable_type = self.table.insertable_data_ref(self.workspace).unwrap();
        let formatted_generics = self.table.formatted_table_buildable_generics(self.database);

        for column in self.table.insertable_columns(self.database) {
            let column_path = column.column_path(self.database);
            let column_type = column.rust_type(self.workspace, self.database).unwrap();

            maybe_get_column_impls.push(InternalToken::new()
                .private()
                .stream(quote! {
                    impl #formatted_generics #maybe_get_column<#column_path> for #builder_ident #formatted_generics {
                        fn maybe_get_column(&self) -> Option<&#column_type> {
                            <#insertable_type as #maybe_get_column<#column_path>>::maybe_get_column(
                                &self.#insertable_ident
                            )
                        }
                    }
                })
                .implemented_trait(maybe_get_column.clone().into())
                .build()
                .unwrap());
        }

        maybe_get_column_impls
    }

    /// Returns the implementation of the `MaybeGetColumn` trait for all
    /// ancestors of the builder struct.
    pub(crate) fn ancestor_maybe_get_column_impls(
        &self,
    ) -> impl Iterator<Item = InternalToken> + '_ {
        self.table.ancestral_extended_tables(self.database).into_iter().flat_map(
            move |ancestor_table| {
                let builder_ident = self.table.table_buildable_ident();
                let maybe_get_column = self
                    .workspace
                    .external_trait("MaybeGetColumn")
                    .expect("Failed to get MaybeGetColumn trait");

                let table_to_get_to_ancestor =
                    self.table.extended_table_to(self.database, ancestor_table).expect(&format!(
                        "Failed to get extended table path from {} to ancestor table {}",
                        self.table.table_schema_doc_path(),
                        ancestor_table.table_schema_doc_path()
                    ));
                let extended_table_snake_ident =
                    table_to_get_to_ancestor.table_singular_snake_ident();
                let extended_table_camel_ident =
                    table_to_get_to_ancestor.table_singular_camel_ident();
                    let formatted_generics = self.table.formatted_table_buildable_generics(self.database);

                let schema_module = ancestor_table.schema_module(self.workspace).unwrap();

                ancestor_table
                    .insertable_columns(self.database)
                    .map(move |column| {
                        let column_path = column.column_path(self.database);
                        let column_type = column.rust_type(self.workspace, self.database).unwrap();

                        InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl #formatted_generics #maybe_get_column<#column_path> for #builder_ident #formatted_generics
                        where
                            #extended_table_camel_ident: #maybe_get_column<#column_path>
                        {
                            fn maybe_get_column(&self) -> Option<&#column_type> {
                                <#extended_table_camel_ident as #maybe_get_column<#column_path>>::maybe_get_column(
                                    &self.#extended_table_snake_ident
                                )
                            }
                        }
                    })
                    .implemented_trait(maybe_get_column.clone().into())
                    .internal_module(schema_module.clone())
                    .build()
                    .unwrap()
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            },
        )
    }
}
