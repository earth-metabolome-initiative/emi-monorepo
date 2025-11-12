//! Submodule implementing the generation of the `*ValueSettable` trait for the
//! `TableInsertable` struct.

use std::borrow::Borrow;

use quote::quote;
use sql_relations::traits::InheritableDatabaseLike;
use sql_traits::traits::ForeignKeyLike;
use synql_checks::prelude::CheckConstraintSynLike;
use synql_core::{
    prelude::Builder,
    structs::{InternalData, InternalToken, MethodBuilder},
    traits::ColumnSynLike,
};
use synql_insertable_key_settable::prelude::*;

use crate::{structs::TableInsertable, traits::TableInsertableLike};

impl<'table, T: TableInsertableLike + ?Sized> TableInsertable<'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Returns the implementation of the `*InsertableKeySettable` trait for the
    /// insertable struct.
    pub(crate) fn insertable_key_settable_impl(&self) -> InternalToken {
        let trait_ref = self
            .table
            .insertable_key_settable_trait_ref(self.workspace)
            .expect("Failed to get InsertableKeySettable trait ref");
        let identiable = self
            .workspace
            .external_trait("Identifiable")
            .expect("Failed to get Identifiable trait");
        let data: InternalData = self.clone().into();
        trait_ref
            .impl_for_type(&data.into())
            .methods(self.table.insertable_key_settable_foreign_keys(self.database).map(
                |foreign_key| {
                    let host_column = foreign_key.host_column(self.database).unwrap();
                    let builder: MethodBuilder = trait_ref
                        .method(&host_column.column_snake_name())
                        .expect("Failed to get ValueSettable method definition")
                        .clone()
                        .into();
                    let snake_case_ident = host_column.column_snake_ident();
                    let contextual_columns = &[host_column.borrow()];
                    let check_constraints = host_column
                        .insertable_key_settable_check_constraints(self.database)
                        .map(|check_constraint| {
                            check_constraint.to_syn(
                                self.database,
                                self.workspace,
                                contextual_columns,
                            )
                        })
                        .collect::<Vec<_>>();

                    let (maybe_deref, maybe_clone) = if host_column.supports_copy(self.database, self.workspace) {
                        (Some(quote! {*}), None)
                    } else {
                        (None, Some(quote! { .clone() }))
                    };

                    builder
                        .make_mut_self()
                        .unwrap()
                        .body(
                            InternalToken::new()
                                .stream(if check_constraints.is_empty() {
                                    quote! {
                                        use #identiable;
                                        self.#snake_case_ident = Some(#maybe_deref #snake_case_ident.id() #maybe_clone);
                                        self
                                    }
                                } else {
                                    quote! {
                                        use #identiable;
                                        let #snake_case_ident = #maybe_deref #snake_case_ident.id() #maybe_clone;
                                        #(#check_constraints)*
                                        self.#snake_case_ident = Some(#snake_case_ident);
                                        Ok(self)
                                    }
                                })
                                .inherits(check_constraints)
                                .build()
                                .unwrap(),
                        )
                        .build()
                        .unwrap()
                },
            ))
            .unwrap()
            .try_into()
            .expect("Failed to create ValueSettable impl")
    }
}
