//! Submodule defining the `TableModel` struct.

use quote::{ToTokens, quote};
use synql_core::{
    prelude::{Builder, ForeignKeyLike},
    structs::{
        Decorator, Derive, InternalCrate, InternalData, InternalModule, InternalStruct,
        InternalToken,
    },
    traits::ColumnSynLike,
};

use crate::traits::{TableModelLike, column_model_like::ColumnModelLike};

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

    fn diesel_derives(&self) -> Derive<'data> {
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

    fn belongs_to_decorators(&self) -> Vec<Decorator<'data>> {
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
                        .unwrap()
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

    fn primary_key_decorator(&self) -> Option<Decorator<'data>> {
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
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalData<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        let table_schema = table_model
            .table
            .schema_module(table_model.workspace)
            .expect("Failed to get the table schema module");
        let snake_case_ident = table_model.table.table_snake_ident();
        let mut builder = InternalData::new()
            .public()
            .name(table_model.table.table_singular_camel_name())
            .expect("Failed to set name")
            .derive(table_model.diesel_derives())
            .expect("Failed to add derives")
            .decorator(
                Decorator::new()
                    .token(
                        InternalToken::new()
                            .private()
                            .stream(quote! {diesel(table_name = #table_schema::#snake_case_ident)})
                            .internal_module(table_schema)
                            .unwrap()
                            .build()
                            .unwrap(),
                    )
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .unwrap()
            .decorators(table_model.belongs_to_decorators())
            .unwrap()
            .decorators(table_model.primary_key_decorator())
            .unwrap()
            .variant(
                InternalStruct::new()
                    .attributes(
                        table_model
                            .table
                            .columns(table_model.database)
                            .map(|col| col.attribute(table_model.database, table_model.workspace)),
                    )
                    .expect("Failed to set attributes")
                    .build()
                    .expect("Failed to build struct variant"),
            );

        if let Some(documentation) = table_model.table.table_doc(table_model.database) {
            builder = builder.documentation(documentation).expect("Failed to set documentation");
        }

        builder.build().expect("Failed to build table model")
    }
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalModule<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        InternalModule::new()
            .public()
            .name("model")
            .expect("Failed to set the module name")
            .documentation(format!(
                "Submodule providing the [`{}`] data model for the `{}` table.",
                table_model.table.table_singular_camel_name(),
                table_model.table.table_name()
            ))
            .expect("Failed to set the module documentation")
            .public()
            .data(table_model.into())
            .expect("Failed to add the internal data to module")
            .build()
            .expect("Failed to convert internal data into internal module")
    }
}

impl<'data, 'table, T> From<TableModel<'data, 'table, T>> for InternalCrate<'data>
where
    T: TableModelLike + ?Sized,
{
    fn from(table_model: TableModel<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(table_model.table.table_model_crate_name())
            .expect("Failed to set the crate name")
            .documentation(format!(
                "Crate providing the [`{}`] data model for the `{}` table.",
                table_model.table.table_singular_camel_name(),
                table_model.table.table_name()
            ))
            .expect("Failed to set the crate documentation")
            .module(table_model.into())
            .expect("Failed to add internal module to internal crate")
            .build()
            .expect("Failed to convert internal data into internal crate")
    }
}

impl<'data, 'table, T> ToTokens for TableModel<'data, 'table, T>
where
    T: TableModelLike + ?Sized,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let internal_data: InternalData<'data> = InternalData::from(*self);
        internal_data.to_tokens(tokens);
    }
}
