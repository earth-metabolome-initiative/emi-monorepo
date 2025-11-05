//! Submodule implementing the generation of the `*ValueSettable` trait for the
//! `TableBuildable` struct.

use sql_relations::traits::InheritableDatabaseLike;
use synql_core::{
    prelude::Builder,
    structs::{InternalData, InternalToken, MethodBuilder},
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
        let insertable_ident = self.table.insertable_attribute_ident();
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
}
