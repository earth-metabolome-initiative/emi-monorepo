//! Submodule implementing the generation of the `SetColumn` and `TrySetColumn`
//! traits for the `TableInsertable` struct.

use std::borrow::Borrow;

use quote::quote;
use sql_relations::traits::InheritableDatabaseLike;
use sql_traits::traits::{CheckConstraintLike, ColumnLike};
use synql_attributes::traits::TableAttributesLike;
use synql_checks::prelude::CheckConstraintSynLike;
use synql_core::{
    prelude::Builder,
    structs::{InternalData, InternalToken},
    traits::ColumnSynLike,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::{structs::TableInsertable, traits::TableInsertableLike};

impl<'table, T: TableInsertableLike + ?Sized> TableInsertable<'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Returns the implementations of the `SetColumn` or `TrySetColumn` traits
    /// for all insertable columns in the table.
    pub(crate) fn set_column_impls(&self) -> Vec<InternalToken> {
        let mut set_column_impls = Vec::new();
        let data: InternalData = self.clone().into();

        let set_column_trait =
            self.workspace.external_trait("SetColumn").expect("Failed to get SetColumn trait");

        let try_set_column_trait = self
            .workspace
            .external_trait("TrySetColumn")
            .expect("Failed to get TrySetColumn trait");

        let table_attributes =
            self.table.attributes_ref(self.workspace).expect("Failed to retrieve table attributes");

        let _validation_error = self
            .workspace
            .external_type(&syn::parse_quote!(validation_errors::prelude::ValidationError))
            .expect("Failed to get ValidationError type ref")
            .set_generic_field(&syn::parse_quote!(FieldName), table_attributes.into())
            .expect("Failed to set generic field for ValidationError");

        for column in self.table.insertable_columns(self.database) {
            let column_path = column.column_path(self.database);
            let column_type = column.rust_type(self.workspace, self.database).unwrap();
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

            let has_check_constraints = !check_constraints.is_empty();

            if has_check_constraints {
                // Implement TrySetColumn
                let set_column_impl = InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl #try_set_column_trait<#column_path> for #data {
                            fn try_set(mut self, #snake_case_ident: #column_type) -> Result<Self, > {
                                #(#check_constraints)*
                                self.#snake_case_ident = Some(#snake_case_ident);
                                Ok(self)
                            }
                        }
                    })
                    .implemented_trait(try_set_column_trait.clone().into())
                    .inherits(check_constraints.clone())
                    .build()
                    .unwrap();

                set_column_impls.push(set_column_impl);
            } else {
                // Implement SetColumn
                let set_column_impl = InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl #set_column_trait<#column_path> for #data {
                            fn set(mut self, #snake_case_ident: #column_type) -> Self {
                                self.#snake_case_ident = Some(#snake_case_ident);
                                self
                            }
                        }
                    })
                    .implemented_trait(set_column_trait.clone().into())
                    .build()
                    .unwrap();

                set_column_impls.push(set_column_impl);
            }
        }

        set_column_impls
    }
}
