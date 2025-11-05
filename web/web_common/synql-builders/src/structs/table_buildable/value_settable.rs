//! Submodule implementing the generation of the `*ValueSettable` trait for the
//! `TableBuildable` struct.

use quote::quote;
use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{InternalData, InternalToken, MethodBuilder, WhereClause},
    traits::ColumnSynLike,
};

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'data, 'table, T: TableBuildableLike + ?Sized> TableBuildable<'data, 'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Returns the implementation of the `*ValueSettable` trait for the
    /// insertable struct.
    pub(crate) fn value_settable_impl(&self) -> InternalToken<'data> {
        let trait_ref = self
            .table
            .value_settable_trait_ref(self.workspace)
            .expect("Failed to get ValueSettable trait ref");
        let data: InternalData<'data> = self.clone().into();
        let insertable_ident = self.table.table_singular_snake_ident();
        trait_ref
            .impl_for_type(&data.into())
            .methods(self.table.value_settable_columns(self.database).map(|column| {
                let builder: MethodBuilder = trait_ref
                    .method(&column.column_snake_name())
                    .expect("Failed to get ValueSettable method definition")
                    .clone()
                    .into();
                let method_ident = column.column_snake_ident();
                builder
                    .make_mut_self()
                    .unwrap()
                    .body(quote::quote! {
                        self.#insertable_ident = self.#insertable_ident.#method_ident(#method_ident)?;
                        Ok(self)
                    })
                    .build()
                    .unwrap()
            }))
            .unwrap()
            .try_into()
            .expect("Failed to create ValueSettable impl")
    }

    /// Returns the implementation of the `*ValueSettable` trait for all
    /// ancestors of the model struct.
    pub(crate) fn ancestor_value_settable_impls(
        &self,
    ) -> impl Iterator<Item = InternalToken<'data>> + '_ {
        self.table.ancestral_extended_tables(self.database).into_iter().map(move |ancestor_table| {
            let trait_ref = ancestor_table
                .value_settable_trait_ref(self.workspace)
                .expect(&format!(
                    "Failed to get ValueSettable trait ref for ancestor table {}",
                    ancestor_table.table_schema_doc_path()
                ));
            let data: InternalData<'data> = self.clone().into();
            let table_to_get_to_ancestor = self.table.extended_table_to(self.database, ancestor_table).expect(&format!(
                "Failed to get extended table path from {} to ancestor table {}",
                self.table.table_schema_doc_path(),
                ancestor_table.table_schema_doc_path()
            ));
            let extended_table_snake_ident = table_to_get_to_ancestor.table_singular_snake_ident();
            let extended_table_camel_ident = table_to_get_to_ancestor.table_singular_camel_ident();
            trait_ref
                .impl_for_type(&data.into())
                .where_clause(
                    WhereClause::new()
                        .left(quote!{#extended_table_camel_ident})
                        .right(quote!{#trait_ref})
                        .build()
                        .unwrap()
                ).unwrap()
                .methods(ancestor_table.value_settable_columns(self.database).map(|column| {
                    let builder: MethodBuilder = trait_ref
                        .method(&column.column_snake_name())
                        .expect("Failed to get ValueSettable method definition")
                        .clone()
                        .into();
                    let method_ident = column.column_snake_ident();
                    builder
                        .make_mut_self()
                        .unwrap()
                        .body(quote::quote! {
                            self.#extended_table_snake_ident = self.#extended_table_snake_ident.#method_ident(#method_ident)?;
                            Ok(self)
                        })
                        .build()
                        .unwrap()
                }))
                .unwrap()
                .try_into()
                .expect("Failed to create ValueSettable impl")
        })
    }
}
