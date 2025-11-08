//! Submodule defining the `TableModel` struct.

use quote::{ToTokens, quote};
use synql_core::{
    prelude::{Builder, ForeignKeyLike},
    structs::{Decorator, Derive, InternalData, InternalToken},
    traits::ColumnSynLike,
};

mod into_crate;
mod into_data;
mod into_module;
use crate::traits::TableModelLike;

#[derive(Debug)]
/// Struct representing a SynQL table model.
pub struct TableModel<'data, 'table, T: TableModelLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableModelLike + ?Sized> Clone for TableModel<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableModelLike + ?Sized> Copy for TableModel<'data, 'table, T> {}

impl<'data, 'table, T: TableModelLike + ?Sized> TableModel<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    fn diesel_derives(&self) -> Derive {
        let mut traits = vec![
            self.workspace.external_trait("Selectable").expect("Failed to get Selectable trait"),
        ];

        if self.table.has_non_primary_key_columns(self.database) {
            traits.push(
                self.workspace
                    .external_trait("AsChangeset")
                    .expect("Failed to get AsChangeset trait"),
            );
        }

        if !self.table.has_generated_columns(self.database) {
            traits.push(
                self.workspace
                    .external_trait("Insertable")
                    .expect("Failed to get Insertable trait"),
            );
        }

        if self.table.has_primary_key(self.database) {
            traits.push(
                self.workspace.external_trait("Queryable").expect("Failed to get Queryable trait"),
            );
            traits.push(
                self.workspace
                    .external_trait("Identifiable")
                    .expect("Failed to get Identifiable trait"),
            );
        }

        if self
            .table
            .singleton_foreign_keys(self.database)
            .any(|fk| !fk.is_composite(self.database))
        {
            traits.push(
                self.workspace
                    .external_trait("Associations")
                    .expect("Failed to get Associations trait"),
            );
        }

        Derive::new()
            .add_traits(traits)
            .expect("Failed to add Selectable trait to derive")
            .build()
            .expect("Failed to build Selectable derive")
    }

    fn belongs_to_decorators(&self) -> Vec<Decorator> {
        let mut decorators = Vec::new();

        for foreign_key in self.table.singleton_foreign_keys(self.database) {
            let columns = foreign_key.host_columns(self.database).collect::<Vec<_>>();
            let [column] = columns.as_slice() else {
                continue;
            };
            let column_ident = column.column_snake_ident();

            let referenced_table = foreign_key.referenced_table(self.database);
            let referenced_table_model = referenced_table
                .model_ref(self.workspace)
                .expect("Failed to get the referenced table model for belongs_to decorator");

            let decorator = Decorator::new()
                .token(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            diesel(belongs_to(#referenced_table_model, foreign_key = #column_ident))
                        })
                        .data(referenced_table_model.into())
                        .build()
                        .unwrap(),
                )
                .unwrap()
                .build()
                .unwrap();

            decorators.push(decorator);
        }

        decorators
    }

    fn primary_key_decorator(&self) -> Option<Decorator> {
        if !self.table.has_primary_key(self.database) {
            return None;
        }

        let pk_idents = self.table.primary_key_idents(self.database);

        Some(
            Decorator::new()
                .token(
                    InternalToken::new()
                        .private()
                        .stream(quote! {
                            diesel(primary_key( #(#pk_idents),* ))
                        })
                        .build()
                        .unwrap(),
                )
                .unwrap()
                .build()
                .unwrap(),
        )
    }

    fn extension_of_impls(&self) -> Vec<InternalToken> {
        let extension_of_trait =
            self.workspace.external_trait("ExtensionOf").expect("Failed to get ExtensionOf trait");
        let read_trait = self.workspace.external_trait("Read").expect("Failed to get Read trait");
        let identifiable_trait = self
            .workspace
            .external_trait("Identifiable")
            .expect("Failed to get Identifiable trait");
        let table_model = self.table.table_singular_camel_ident();

        let mut extension_of_impls = vec![
            InternalToken::new()
                .private()
                .stream(quote! {
                    impl<C> #extension_of_trait<#table_model, C> for #table_model {
                        type ExtendedType<'data> = &'data Self
                        where
                            Self: 'data;

                        fn ancestor(
                            &self,
                            _connection: &mut C,
                        ) -> Result<Self::ExtendedType<'_>, diesel::result::Error> {
                            Ok(self)
                        }
                    }
                })
                .implemented_trait(extension_of_trait.clone().into())
                .build()
                .unwrap(),
        ];

        for extended_table in self.table.ancestral_extended_tables(self.database) {
            let extended_table_model = extended_table
                .model_ref(self.workspace)
                .expect("Failed to get the extended table model for ExtensionOf impl");

            extension_of_impls.push(
                InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl<C> #extension_of_trait<#extended_table_model, C> for #table_model
                        where
                            #extended_table_model: #read_trait<C>,
                        {
                            type ExtendedType<'data> = #extended_table_model
                            where
                                Self: 'data;

                            fn ancestor(
                                &self,
                                connection: &mut C,
                            ) -> Result<Self::ExtendedType<'_>, diesel::result::Error> {
                                use #identifiable_trait;
                                use #read_trait;
                                #extended_table_model::read(self.id(), connection)
                            }
                        }
                    })
                    .implemented_trait(extension_of_trait.clone().into())
                    .employed_traits([read_trait.clone().into(), identifiable_trait.clone().into()])
                    .build()
                    .unwrap(),
            );
        }

        extension_of_impls
    }
}

impl<'data, 'table, T> ToTokens for TableModel<'data, 'table, T>
where
    T: TableModelLike + ?Sized,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_data: InternalData = InternalData::from(*self);
        internal_data.to_tokens(tokens);
    }
}
