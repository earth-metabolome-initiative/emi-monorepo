//! Submodule implementing the `From` trait to convert a `SchemaMacro` into
//! an `InternalToken`.

use quote::quote;
use sql_relations::prelude::*;
use sql_traits::traits::ForeignKeyLike;
use synql_core::{
    prelude::{Builder, ColumnLike},
    structs::InternalToken,
    traits::{ColumnSynLike, TableSynLike as _},
};

use crate::{structs::SchemaMacro, traits::TableSchema};

impl<'table, T> From<SchemaMacro<'table, T>> for InternalToken
where
    T: synql_core::traits::TableSynLike,
{
    fn from(schema_macro: SchemaMacro<'table, T>) -> Self {
        let mut columns = Vec::new();
        let mut column_types = Vec::new();

        let mut builder = InternalToken::new();

        for column in schema_macro.table.columns(schema_macro.database) {
            let column_attribute = column.column_snake_ident();
            let column_type = column
                .diesel_type(schema_macro.workspace, schema_macro.database)
                .unwrap_or_else(|| {
                    panic!(
                        "Could not find diesel type `{}` for column `{}.{}`",
                        column.normalized_data_type(schema_macro.database),
                        schema_macro.table.table_name(),
                        column.column_name(),
                    )
                });

            // Collect the external type reference for the column
            if let Some(external_type) =
                column.external_postgres_type(schema_macro.workspace, schema_macro.database)
            {
                column_types.push(external_type.into());
            }

            let sql_name = if column.has_snake_case_column_name() {
                None
            } else {
                let original_column_name = column.column_name();
                Some(quote! {#[sql_name = #original_column_name]})
            };

            let documentation = if let Some(doc) = column.column_doc(schema_macro.database) {
                Some(quote! {
                    #[doc = #doc]
                })
            } else {
                None
            };

            columns.push(quote! {
                #documentation
                #sql_name
                #column_attribute -> #column_type
            });
        }

        let primary_key = if schema_macro.table.has_primary_key(schema_macro.database) {
            let primary_keys = schema_macro.table.primary_key_idents(schema_macro.database);
            Some(quote! {
                (#(#primary_keys),*)
            })
        } else {
            None
        };

        let documentation = if let Some(doc) = schema_macro.table.table_doc(schema_macro.database) {
            Some(quote! {
                #[doc = #doc]
            })
        } else {
            None
        };

        let sql_name = if schema_macro.table.has_snake_case_table_name() {
            None
        } else {
            let original_table_name = schema_macro.table.table_name();
            Some(quote! {#[sql_name = #original_table_name]})
        };

        let maybe_schema = if let Some(schema_name) = schema_macro.table.table_schema() {
            Some(quote! {#schema_name.})
        } else {
            None
        };
        let table_name_ident = schema_macro.table.table_snake_ident();

        let mut token_stream = quote! {
            diesel::table! {
                #documentation
                #sql_name
                #maybe_schema #table_name_ident #primary_key {
                    #(#columns),*
                }
            }
        };

        for referenced_table in schema_macro.table.non_self_referenced_tables(schema_macro.database)
        {
            let crate_ref = schema_macro
                .workspace
                .internal_crate(&referenced_table.table_schema_crate_name())
                .expect(&format!(
                    "Failed to find the referenced table struct: {}",
                    referenced_table.table_name()
                ));
            let crate_ident = crate_ref.ident();
            let referenced_table_name_ident = referenced_table.table_snake_ident();
            token_stream.extend(quote! {
                use #crate_ident::schema::#referenced_table_name_ident;
                diesel::allow_tables_to_appear_in_same_query!(#table_name_ident, #referenced_table_name_ident);
            });

            builder = builder.internal_module(
                referenced_table.schema_module(schema_macro.workspace).expect(&format!(
                    "Failed to get the schema module for the referenced table `{}` while building the schema macro for table `{}`.",
                    referenced_table.table_name(),
                    schema_macro.table.table_name()
                )),
            );
        }

        for foreign_key in
            schema_macro.table.non_self_referential_singleton_foreign_keys(schema_macro.database)
        {
            let host_columns = foreign_key.host_columns(schema_macro.database).collect::<Vec<_>>();
            let [host_column] = host_columns.as_slice() else {
                continue;
            };

            let referenced_table = foreign_key.referenced_table(schema_macro.database);
            let referenced_table_name_ident = referenced_table.table_snake_ident();
            let host_column_ident = host_column.column_snake_ident();

            token_stream.extend(quote! {
                diesel::joinable!(#table_name_ident -> #referenced_table_name_ident (#host_column_ident));
            });
        }

        let mut macros = vec![
            schema_macro.workspace.external_macro("table").expect("Failed to find the table macro"),
        ];

        if schema_macro.table.has_non_self_referential_foreign_keys(schema_macro.database) {
            let allow_tables_to_appear_in_same_query = schema_macro
                .workspace
                .external_macro("allow_tables_to_appear_in_same_query")
                .expect("Failed to find the allow_tables_to_appear_in_same_query macro");

            macros.push(allow_tables_to_appear_in_same_query);
        }

        if schema_macro.table.is_extension(schema_macro.database) {
            let table_is_extension_of_trait = schema_macro
                .workspace
                .external_trait("TableIsExtensionOf")
                .expect("Failed to find the TableIsExtensionOf trait");

            for extended_table in
                schema_macro.table.ancestral_extended_tables(schema_macro.database)
            {
                let schema_module = extended_table.schema_module(schema_macro.workspace).unwrap();
                let extended_table_ident = extended_table.table_snake_ident();
                token_stream.extend(quote! {
                    impl #table_is_extension_of_trait<#schema_module::#extended_table_ident::table> for #table_name_ident::table {}
                });
                builder = builder.internal_module(schema_module);
            }
            builder = builder.employed_trait(table_is_extension_of_trait.into());
        }

        if schema_macro.table.has_vertical_same_as(schema_macro.database) {
            let vertical_same_as = schema_macro
                .workspace
                .external_trait("VerticalSameAs")
                .expect("Failed to find the VerticalSameAs trait");

            for fk in schema_macro.table.vertical_same_as_foreign_keys(schema_macro.database) {
                let referenced_table = fk.referenced_table(schema_macro.database);
                let referenced_table_ident = referenced_table.table_snake_ident();
                let referenced_schema_module =
                    referenced_table.schema_module(schema_macro.workspace).unwrap();
                let (host_column, referenced_column) =
                    fk.vertical_same_as_column_pair(schema_macro.database).unwrap();
                let host_column_ident = host_column.column_snake_ident();
                let referenced_column_ident = referenced_column.column_snake_ident();
                token_stream.extend(quote! {
                    impl #vertical_same_as<#referenced_schema_module::#referenced_table_ident::#referenced_column_ident> for #table_name_ident::#host_column_ident {}
                });
            }

            builder = builder.employed_trait(vertical_same_as.into());
        }

        builder
            .public()
            .external_macros(macros)
            .datas(column_types)
            .stream(token_stream)
            .build()
            .expect(&format!(
                "Failed to build the internal token for the schema macro of table `{}`",
                schema_macro.table.table_name()
            ))
    }
}
