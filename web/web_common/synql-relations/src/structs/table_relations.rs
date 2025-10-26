//! Submodule defining the `TableRelations` struct.

use quote::ToTokens;
use syn::Ident;
use synql_core::{
    prelude::{Builder, ForeignKeyLike},
    structs::{
        Argument, DataVariantRef, InternalCrate, InternalModule, InternalToken, InternalTrait,
        Method,
    },
    traits::ForeignKeySynLike,
};
use synql_models::traits::TableModelLike;

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
}

impl<'data, 'table, T> From<TableRelations<'data, 'table, T>> for InternalTrait<'data>
where
    T: TableRelationsLike + ?Sized,
{
    fn from(table_relation: TableRelations<'data, 'table, T>) -> Self {
        let connection_generic =
            DataVariantRef::generic(Ident::new("C", proc_macro2::Span::call_site()));
        InternalTrait::new()
            .public()
            .name(table_relation.table.table_relations_trait_name())
            .expect("Failed to set the internal trait name")
            .documentation(format!(
                "Trait providing relations for the `{}` table.",
                table_relation.table.table_name()
            ))
            .expect("Failed to set the internal trait documentation")
            .methods(table_relation.table.foreign_keys(table_relation.database).map(|fk| {
                let referenced_table = fk.referenced_table(table_relation.database);
                let referenced_table_model =
                    referenced_table.model_ref(table_relation.workspace).expect(
                        "Failed to get the model ref for the referenced table of the foreign key",
                    );
                Method::new()
                    .private()
                    .name(fk.foreign_key_getter_name(table_relation.database))
                    .expect("Failed to set the method name")
                    .body(InternalToken::new().private().stream(quote::quote! {}).build().unwrap())
                    .add_argument(
                        Argument::new()
                            .name("connection")
                            .expect("Failed to set the method argument name")
                            .arg_type(connection_generic.clone())
                            .build()
                            .expect("Failed to build the method argument for the connection"),
                    )
                    .expect("Failed to add the method argument for the connection")
                    .return_type(referenced_table_model)
                    .build()
                    .unwrap()
            }))
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
