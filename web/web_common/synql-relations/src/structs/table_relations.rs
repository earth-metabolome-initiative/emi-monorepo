//! Submodule defining the `TableRelations` struct.

use std::borrow::Borrow;

use quote::{ToTokens, quote};
use syn::Ident;
use synql_core::{
    prelude::{Builder, ColumnLike, DatabaseLike, ForeignKeyLike},
    structs::{
        Argument, DataVariantRef, InternalCrate, InternalModule, InternalToken, InternalTrait,
        Method, MethodBuilder, WhereClause,
    },
    traits::{ColumnSynLike, ForeignKeySynLike},
};

use crate::traits::{TRAIT_MODULE_NAME, TableRelationsLike};

#[derive(Debug)]
/// Struct representing a SynQL table relations trait.
pub struct TableRelations<'data, 'table, T: TableRelationsLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableRelationsLike + ?Sized> Clone for TableRelations<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableRelationsLike + ?Sized> Copy for TableRelations<'data, 'table, T> {}

impl<'data, 'table, T: TableRelationsLike + ?Sized> TableRelations<'data, 'table, T> {
    /// Creates a new `TableRelations` instance.
    ///
    /// # Arguments
    ///
    /// * `table` - The table model implementing `TableRelationsLike`.
    /// * `workspace` - The workspace where the table is defined.
    /// * `database` - The database connection to use to query the table model.
    pub fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    fn init_method_builders(
        &self,
        foreign_key: &<T::DB as DatabaseLike>::ForeignKey,
    ) -> (Argument<'data>, MethodBuilder<'data>) {
        let connection_generic =
            DataVariantRef::generic(Ident::new("C", proc_macro2::Span::call_site()));
        let referenced_table: &T = foreign_key.referenced_table(self.database).borrow();
        let referenced_table_model = referenced_table
            .model_ref(self.workspace)
            .expect("Failed to get the model ref for the referenced table of the foreign key");
        let connection_argument = Argument::new()
            .name("connection")
            .expect("Failed to set the method argument name")
            .arg_type(connection_generic.mutable_reference(None))
            .build()
            .expect("Failed to build the method argument for the connection");

        (
            connection_argument.clone(),
            Method::new()
                .private()
                .documentation(format!(
                    "Returns the {} referenced to by the foreign key [{}].",
                    referenced_table_model.documentation_path(),
                    foreign_key.documentation_repr(self.database)
                ))
                .expect("Failed to set the method documentation")
                .error_documentation("* If the provided DB connection fails.")
                .name(foreign_key.foreign_key_getter_name(self.database))
                .expect("Failed to set the method name")
                .add_argument(
                    Argument::new()
                        .name("self")
                        .unwrap()
                        .arg_type(DataVariantRef::self_type(None).reference(None))
                        .build()
                        .expect("Failed to build self argument"),
                )
                .expect("Failed to add self argument")
                .add_argument(connection_argument)
                .expect("Failed to add the method argument for the connection")
                .return_type(DataVariantRef::diesel_result(
                    if foreign_key.is_always_enforced(self.database) {
                        referenced_table_model.into()
                    } else {
                        DataVariantRef::option(referenced_table_model.into())
                    },
                )),
        )
    }

    fn read_based_method(
        &self,
        foreign_key: &<T::DB as DatabaseLike>::ForeignKey,
    ) -> Method<'data> {
        let referenced_table: &T = foreign_key.referenced_table(self.database).borrow();
        let referenced_table_model = referenced_table
            .model_ref(self.workspace)
            .expect("Failed to get the model ref for the referenced table of the foreign key");
        let read_trait =
            self.workspace.external_trait("Read").expect("Failed to get Read trait from workspace");
        let optional_trait = self
            .workspace
            .external_trait("OptionalExtension")
            .expect("Failed to get OptionalExtension trait from workspace");

        let (connection_argument, method_builder) = self.init_method_builders(foreign_key);
        let connection_generic = connection_argument.arg_type().dereference();
        let connection_ident = connection_argument.ident();
        let mut optional_host_columns_retrieval = Vec::new();
        let mut host_column_idents = Vec::new();
        for host_column in foreign_key.host_columns(self.database) {
            let host_column_ident = host_column.column_snake_ident();
            match (
                host_column.supports_copy(self.database, self.workspace),
                host_column.is_nullable(),
            ) {
                (true, false) => {
                    host_column_idents.push(quote! { self.#host_column_ident });
                }
                (true, true) => {
                    host_column_idents.push(quote! { #host_column_ident });
                    optional_host_columns_retrieval.push(quote! {
                        let Some(#host_column_ident) = self.#host_column_ident else {
                            return Ok(None),
                        };
                    });
                }
                (false, false) => {
                    host_column_idents.push(quote! { self.#host_column_ident.clone() });
                }
                (false, true) => {
                    host_column_idents.push(quote! { #host_column_ident });
                    optional_host_columns_retrieval.push(quote! {
                        let #host_column_ident = self.#host_column_ident.clone() else {
                            return Ok(None);
                        };
                    });
                }
            }
        }

        let formatted_host_columns = if host_column_idents.len() == 1 {
            host_column_idents[0].clone()
        } else {
            quote! { (#(#host_column_idents),*) }
        };

        let (maybe_use_optional, optional) = if !foreign_key.is_always_enforced(self.database) {
            (Some(quote! {use #optional_trait;}), Some(quote! {.optional()}))
        } else {
            (None, None)
        };

        method_builder
            .add_where_clause(
                WhereClause::new()
                    .left(referenced_table_model.clone())
                    .right(
                        InternalToken::new()
                            .private()
                            .stream(quote! {#read_trait<#connection_generic>})
                            .external_trait(read_trait.clone())
                            .unwrap()
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .body(
                InternalToken::new()
                    .private()
                    .stream(quote::quote! {
                        use #read_trait;
                        #maybe_use_optional
                        #(#optional_host_columns_retrieval)*
                        #referenced_table_model::read(
                            #formatted_host_columns,
                            #connection_ident
                        )#optional
                    })
                    .external_traits([read_trait, optional_trait])
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T> From<TableRelations<'data, 'table, T>> for InternalTrait<'data>
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'data, 'table, T>) -> Self {
        InternalTrait::new()
            .public()
            .name(table_relation.table.table_relations_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(format!(
                "Trait providing relations for the `{}` table.",
                table_relation.table.table_name()
            ))
            .expect("Failed to set the internal trait documentation")
            .generic(syn::parse_quote! {C})
            .unwrap()
            .methods(table_relation.table.foreign_keys(table_relation.database).map(
                |fk: &<T::DB as DatabaseLike>::ForeignKey| {
                    if fk.is_referenced_primary_key(table_relation.database) {
                        table_relation.read_based_method(fk)
                    } else {
                        todo!()
                    }
                },
            ))
            .expect("Failed to set the internal trait methods")
            .build()
            .expect("Failed to convert internal trait builder into internal trait")
    }
}

impl<'data, 'table, T> From<TableRelations<'data, 'table, T>> for InternalModule<'data>
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'data, 'table, T>) -> Self {
        let model_ref = table_relation
            .table
            .model_ref(table_relation.workspace)
            .expect("Failed to get the model ref for the table relations");

        InternalModule::new()
            .public()
            .name(TRAIT_MODULE_NAME)
            .expect("Failed to set the module name")
            .documentation(format!(
                "Submodule providing the [`{}`] trait for the [`{}`]({model_ref}) struct and the `{}` table.",
                table_relation.table.table_relations_trait_name(),
                model_ref.data().name(),
                table_relation.table.table_name()
            ))
            .expect("Failed to set the module documentation")
            .public()
            .internal_trait(table_relation.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}

impl<'data, 'table, T> From<TableRelations<'data, 'table, T>> for InternalCrate<'data>
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(table_relation.table.table_relations_crate_name())
            .expect("Failed to set the crate name")
            .documentation(format!(
                "Crate providing the [`{}`] trait for the `{}` table.",
                table_relation.table.table_relations_trait_name(),
                table_relation.table.table_name()
            ))
            .expect("Failed to set the crate documentation")
            .module(table_relation.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}

impl<'data, 'table, T> ToTokens for TableRelations<'data, 'table, T>
where
    T: TableRelationsLike + ?Sized,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_data: InternalTrait<'data> = InternalTrait::from(*self);
        internal_data.to_tokens(tokens);
    }
}
