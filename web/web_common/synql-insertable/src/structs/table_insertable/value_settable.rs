//! Submodule implementing the generation of the `*ValueSettable` trait for the
//! `TableInsertable` struct.

use std::borrow::Borrow;

use quote::quote;
use sql_relations::traits::InheritableDatabaseLike;
use sql_traits::traits::{CheckConstraintLike, ColumnLike};
use synql_checks::prelude::CheckConstraintSynLike;
use synql_core::{
    prelude::Builder,
    structs::{InternalData, InternalToken, MethodBuilder},
    traits::ColumnSynLike,
};

use crate::{structs::TableInsertable, traits::TableInsertableLike};

impl<'table, T: TableInsertableLike + ?Sized> TableInsertable<'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Returns the implementation of the `*ValueSettable` trait for the
    /// insertable struct.
    pub(crate) fn value_settable_impl(&self) -> InternalToken {
        let trait_ref = self
            .table
            .value_settable_trait_ref(self.workspace)
            .expect("Failed to get ValueSettable trait ref");
        let data: InternalData = self.clone().into();
        trait_ref
            .impl_for_type(&data.into())
            .methods(self.table.value_settable_columns(self.database).map(|column| {
                let builder: MethodBuilder = trait_ref
                    .method(&column.column_snake_name())
                    .expect("Failed to get ValueSettable method definition")
                    .clone()
                    .into();
                let snake_case_ident = column.column_snake_ident();
                let contextual_columns = &[column.borrow()];
                let check_constraints = column
                    .check_constraints(self.database)
                    .filter(|check_constraint| {
                        !check_constraint.is_mutual_nullability_constraint(self.database)
                    })
                    .map(|check_constraint| {
                        check_constraint.to_syn(self.database, self.workspace, contextual_columns)
                    })
                    .collect::<Vec<_>>();
                builder
                    .make_mut_self()
                    .unwrap()
                    .body(
                        InternalToken::new()
                            .stream(if check_constraints.is_empty() {
                                quote! {
                                    self.#snake_case_ident = Some(#snake_case_ident.try_into()?);
                                    Ok(self)
                                }
                            } else {
                                quote! {
                                    let #snake_case_ident = #snake_case_ident.try_into()?;
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
            }))
            .unwrap()
            .try_into()
            .expect("Failed to create ValueSettable impl")
    }
}
