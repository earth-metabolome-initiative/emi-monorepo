//! Submodule implementing the generation of the `*BuildableKeySettable` trait
//! for the `TableBuildable` struct.

use std::borrow::Borrow;

use quote::quote;
use sql_relations::{
    prelude::{ColumnLike, ForeignKeyLike},
    traits::{InheritableDatabaseLike, TriangularSameAsForeignKeyLike, VerticalSameAsColumnLike},
};
use sql_traits::traits::DatabaseLike;
use synql_buildable_key_settable::traits::TableBuildableKeySettableLike;
use synql_core::{
    prelude::Builder,
    structs::{ExternalCrate, InternalData, InternalToken, MethodBuilder, WhereClause},
    traits::ColumnSynLike,
};
use synql_insertable_key_settable::prelude::TableInsertableKeySettableLike;

use crate::{structs::TableBuildable, traits::TableBuildableLike};

impl<'table, T: TableBuildableLike + ?Sized> TableBuildable<'table, T>
where
    T::DB: InheritableDatabaseLike,
{
    /// Returns the implementation of the `*BuildableKeySettable` trait for the
    /// insertable struct.
    pub(crate) fn key_settable_impl(&self) -> InternalToken {
        let trait_ref = self
            .table
            .buildable_key_settable_trait_ref(self.workspace)
            .expect("Failed to get BuildableKeySettable trait ref");
        let insertable_trait_ref = self
            .table
            .insertable_key_settable_trait_ref(self.workspace)
            .expect("Failed to get InsertableKeySettable trait ref");
        let data: InternalData = self.clone().into();
        trait_ref
            .impl_for_type(&data.into())
            .methods(self.table.buildable_key_settable_foreign_keys(self.database).map(|fk| {
                let host_column = fk.host_column(self.database).unwrap();
                let method = trait_ref
                    .method(&host_column.column_snake_name())
                    .expect("Failed to get BuildableKeySettable method definition");
                let builder: MethodBuilder = method.clone().into();
                let return_statement = if method.can_fail() {
                    quote! { Ok(self) }
                } else {
                    quote! { self }
                };
                let body = match fk.triangular_same_as(self.database) {
                    Some(triangular) => {
                        if triangular.is_mandatory() {
                            self.mandatory_triangular_impl(fk)
                        } else {
                            self.discretional_triangular_impl(fk)
                        }
                    }
                    None => self.foreign_key_impl(fk),
                };
                builder
                    .make_mut_self()
                    .unwrap()
                    .body(
                        InternalToken::new()
                            .stream(quote::quote! {
                                use #insertable_trait_ref;
                                #body
                                #return_statement
                            })
                            .inherit(body)
                            .employed_trait(insertable_trait_ref.clone())
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap()
            }))
            .unwrap()
            .try_into()
            .expect("Failed to create BuildableKeySettable impl")
    }

    fn foreign_key_impl(&self, fk: &<T::DB as DatabaseLike>::ForeignKey) -> InternalToken {
        let host_column = fk.host_column(self.database).unwrap();
        let host_column_ident = host_column.column_snake_ident();
        let trait_ref = self
            .table
            .buildable_key_settable_trait_ref(self.workspace)
            .expect("Failed to get BuildableKeySettable trait ref");
        let method = trait_ref
            .method(&host_column.column_snake_name())
            .expect("Failed to get BuildableKeySettable method definition");
        let method_ident = method.ident();
        let question_mark = method.can_fail().then(|| quote! { ? });
        let insertable_ident = self.table.table_singular_snake_ident();
        let maybe_clone = (!host_column.supports_copy(self.database, self.workspace))
            .then(|| quote! { .clone() });

        let vertical_same_as = host_column
            .dominant_vertical_same_as_columns(self.database)
            .into_iter()
            .map(|c| {
                let ancestor_table: &T = c.table(self.database).borrow();
                let ancestor_trait_ref = ancestor_table
                    .buildable_key_settable_trait_ref(self.workspace)
                    .expect("Failed to get BuildableKeySettable trait ref");
                let method = ancestor_trait_ref
                    .method(&c.column_snake_name())
                    .expect(&format!(
                        "Failed to get {} method definition for vertical same column `{}.{}` of `{}.{}`",
                        ancestor_trait_ref.name(),
                        ancestor_table.table_name(),
                        c.column_name(),
                        self.table.table_name(),
                        host_column.column_name()
                    ));
                let method_ident = method.ident();
                let question_mark = method.can_fail().then(|| quote! { ? });
                InternalToken::new()
                    .stream(quote::quote! {
                        use #ancestor_trait_ref;
                        self = self.#method_ident(#host_column_ident #maybe_clone)#question_mark;
                    })
                    .build()
                    .unwrap()
            })
            .collect::<Vec<InternalToken>>();

        InternalToken::new()
            .stream(quote::quote! {
                #(#vertical_same_as)*
                self.#insertable_ident = self.#insertable_ident.#method_ident(#host_column_ident)#question_mark;
            })
            .inherits(vertical_same_as)
            .build()
            .unwrap()
    }

    fn mandatory_triangular_impl(&self, fk: &<T::DB as DatabaseLike>::ForeignKey) -> InternalToken {
        let host_column = fk.host_column(self.database).unwrap();
        let host_column_ident = host_column.column_snake_ident();

        InternalToken::new()
            .stream(quote::quote! {
                self.#host_column_ident = Some(#host_column_ident);
            })
            .build()
            .unwrap()
    }

    fn discretional_triangular_impl(
        &self,
        fk: &<T::DB as DatabaseLike>::ForeignKey,
    ) -> InternalToken {
        let host_column = fk.host_column(self.database).unwrap();
        let host_column_ident = host_column.column_snake_ident();
        let either = ExternalCrate::either_of(None, None);
        let foreign_key_impl = self.foreign_key_impl(fk);
        let mandatory_triangular_impl = self.mandatory_triangular_impl(fk);
        InternalToken::new()
            .stream(quote::quote! {
                match #host_column_ident {
                    #either::Left(#host_column_ident) => {
                        #mandatory_triangular_impl
                    },
                    #either::Right(#host_column_ident) => {
                        #foreign_key_impl
                    }
                }
            })
            .inherits([foreign_key_impl, mandatory_triangular_impl])
            .build()
            .unwrap()
    }

    /// Returns the implementation of the `*BuildableKeySettable` trait for all
    /// ancestors of the model struct.
    pub(crate) fn ancestor_key_settable_impls(&self) -> impl Iterator<Item = InternalToken> + '_ {
        self.table.ancestral_extended_tables(self.database).into_iter().map(move |ancestor_table| {
            let trait_ref = ancestor_table
                .buildable_key_settable_trait_ref(self.workspace)
                .expect(&format!(
                    "Failed to get BuildableKeySettable trait ref for ancestor table {}",
                    ancestor_table.table_schema_doc_path()
                ));
            let data: InternalData = self.clone().into();
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
                .methods(ancestor_table.buildable_key_settable_foreign_keys(self.database).map(|fk| {
                    let host_column = fk.host_column(self.database).unwrap();
                    let method = trait_ref
                        .method(&host_column.column_snake_name())
                        .expect("Failed to get BuildableKeySettable method definition");
                    let builder: MethodBuilder = method
                        .clone()
                        .into();
                    let method_ident = method.ident();
                    let (question_mark, return_statement) = if method.can_fail() {
                        (Some(quote!{?}), quote! { Ok(self) })
                    } else {
                        (None, quote! { self })
                    };
                    builder
                        .make_mut_self()
                        .unwrap()
                        .body(quote::quote! {
                            self.#extended_table_snake_ident = self.#extended_table_snake_ident.#method_ident(#method_ident)#question_mark;
                            #return_statement
                        })
                        .build()
                        .unwrap()
                }))
                .unwrap()
                .try_into()
                .expect("Failed to create BuildableKeySettable impl")
        })
    }
}
