//! Submodule defining the `TableInsertable` struct.

use quote::{ToTokens, quote};
use sql_traits::traits::ColumnLike;
use synql_core::{
    prelude::Builder,
    structs::{Decorator, Derive, InternalCrate, InternalData, InternalModule, InternalToken},
};
use synql_models::traits::ColumnModelLike;

use crate::traits::TableInsertableLike;

#[derive(Debug)]
/// Struct representing a SynQL table insertable.
pub struct TableInsertable<'data, 'table, T: TableInsertableLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableInsertableLike + ?Sized> Clone for TableInsertable<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableInsertableLike + ?Sized> Copy for TableInsertable<'data, 'table, T> {}

impl<'data, 'table, T: TableInsertableLike + ?Sized> TableInsertable<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    /// Returns the derives for the insertable struct.
    fn diesel_derives(&self) -> Derive<'data> {
        let traits = vec![
            self.workspace.external_trait("Insertable").expect("Failed to get Insertable trait"),
        ];

        Derive::new()
            .add_traits(traits)
            .expect("Failed to add Insertable trait to derive")
            .build()
            .expect("Failed to build Insertable derive")
    }

    /// Returns the table decorator for the insertable struct.
    fn table_decorator(&self) -> Decorator<'data> {
        let table_schema = self
            .table
            .schema_module(self.workspace)
            .expect("Failed to get the table schema module");
        let snake_case_ident = self.table.table_snake_ident();

        Decorator::new()
            .token(
                InternalToken::new()
                    .private()
                    .stream(quote! {
                        diesel(table_name = #table_schema::#snake_case_ident)
                    })
                    .internal_module(table_schema)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .build()
            .unwrap()
    }

    /// Returns the insertable columns for the table.
    fn insertable_columns(
        &self,
    ) -> impl Iterator<Item = &'table <T::DB as sql_traits::traits::DatabaseLike>::Column> {
        self.table.columns(self.database).filter(|column| {
            // Exclude auto-generated columns (e.g., serial primary keys)
            !column.is_generated()
        })
    }

    /// Returns the internal data representing the insertable.
    fn insertable_data(&self) -> InternalData<'data> {
        let struct_name = self.table.table_insertable_name();
        let mut data_builder = InternalData::new()
            .public()
            .name(struct_name)
            .expect("Failed to set insertable struct name")
            .derive(self.diesel_derives())
            .expect("Failed to add derives to insertable struct")
            .decorator(self.table_decorator())
            .expect("Failed to add table decorator to insertable struct");

        // Add documentation
        if let Some(table_doc) = self.table.table_doc(self.database) {
            data_builder = data_builder
                .documentation(format!("Insertable struct for {}.", table_doc))
                .expect("Failed to add documentation to insertable struct");
        }

        // Add struct variant with attributes for insertable columns
        let attributes = self
            .insertable_columns()
            .map(|column| column.attribute(self.database, self.workspace))
            .collect::<Vec<_>>();

        data_builder = data_builder.variant(
            synql_core::structs::InternalStruct::new()
                .attributes(attributes)
                .expect("Failed to set attributes")
                .build()
                .expect("Failed to build struct variant")
                .into(),
        );

        data_builder.build().expect("Failed to build insertable data")
    }

    /// Returns the module containing the insertable struct.
    fn insertable_module(&self) -> InternalModule<'data> {
        let module_name = crate::traits::table_insertable_like::INSERTABLE_MODULE_NAME;

        InternalModule::new()
            .name(module_name)
            .expect("Failed to set insertable module name")
            .public()
            .documentation(format!(
                "Submodule providing the [`{}`] insertable struct for the `{}` table.",
                self.table.table_insertable_name(),
                self.table.table_name()
            ))
            .expect("Failed to add documentation to insertable module")
            .data(self.insertable_data())
            .expect("Failed to add insertable struct to insertable module")
            .build()
            .expect("Failed to build insertable module")
    }

    /// Returns the crate containing the insertable module.
    fn insertable_crate(&self) -> InternalCrate<'data> {
        let crate_name = self.table.table_insertable_crate_name();

        InternalCrate::new()
            .name(crate_name)
            .expect("Failed to set insertable crate name")
            .documentation(format!(
                "Crate containing the insertable struct for the `{}` table.",
                self.table.table_name()
            ))
            .expect("Failed to add documentation to insertable crate")
            .module(self.insertable_module())
            .expect("Failed to add insertable module to insertable crate")
            .build()
            .expect("Failed to build insertable crate")
    }
}

impl<'data, 'table, T: TableInsertableLike + ?Sized> From<TableInsertable<'data, 'table, T>>
    for InternalData<'data>
{
    fn from(insertable: TableInsertable<'data, 'table, T>) -> Self {
        insertable.insertable_data()
    }
}

impl<'data, 'table, T: TableInsertableLike + ?Sized> From<TableInsertable<'data, 'table, T>>
    for InternalCrate<'data>
{
    fn from(insertable: TableInsertable<'data, 'table, T>) -> Self {
        insertable.insertable_crate()
    }
}

impl<'data, 'table, T: TableInsertableLike + ?Sized> ToTokens
    for TableInsertable<'data, 'table, T>
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.insertable_data().to_tokens(tokens);
    }
}
