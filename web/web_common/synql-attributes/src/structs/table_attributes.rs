//! Submodule defining the `TableAttributes` struct.
use quote::quote;
use synql_core::{
    prelude::{Builder, ColumnLike},
    structs::{
        InternalCrate, InternalData, InternalDataVariant, InternalEnum, InternalModule,
        InternalToken, InternalVariant,
    },
    traits::ColumnSynLike,
};

use crate::traits::{TableAttributesLike, table_attributes_like::ATTRIBUTES_MODULE_NAME};

#[derive(Debug)]
/// Struct representing a SynQL table attributes enumeration.
pub struct TableAttributes<'data, 'table, T: TableAttributesLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> Clone for TableAttributes<'data, 'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> Copy for TableAttributes<'data, 'table, T> {}

impl<'data, 'table, T: TableAttributesLike + ?Sized> TableAttributes<'data, 'table, T> {
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace<'data>,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    fn display_impl(&self) -> InternalToken<'data> {
        let display_trait = synql_core::structs::ExternalCrate::core()
            .external_trait_ref("Display")
            .expect("Core crate must have Display trait");
        let enum_ident = self.table.table_attributes_ident();
        let variants = self.table.columns(self.database).map(|column| {
            let column_camel_ident = column.column_camel_ident();
            let column_snake_ident = column.column_snake_name();
            quote! {
                Self::#column_camel_ident => write!(f, #column_snake_ident),
            }
        });
        InternalToken::new()
            .private()
            .implemented_trait(display_trait.into())
            .unwrap()
            .stream(quote! {
                impl #display_trait for #enum_ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            #(#variants)*
                        }
                    }
                }
            })
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalEnum<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        InternalEnum::new()
            .variants(attributes.table.columns(attributes.database).map(|column| {
                InternalVariant::new()
                    .name(column.column_camel_ident())
                    .doc(format!(
                        "Attribute representing the `{}.{}` column.",
                        attributes.table.table_name(),
                        column.column_name()
                    ))
                    .expect("Failed to set documentation for attribute variant")
                    .build()
                    .expect("Failed to build attribute variant")
            }))
            .expect("Failed to set variants for attributes enum")
            .build()
            .expect("Failed to build attributes enum")
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalDataVariant<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        let enum_variant: InternalEnum<'data> = attributes.into();
        enum_variant.into()
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalData<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        InternalData::new()
            .name(attributes.table.table_attributes_name())
            .expect("Failed to set attributes enum name")
            .public()
            .documentation(format!(
                "Enumeration of the attributes of the `{}` table.",
                attributes.table.table_name()
            ))
            .expect("Failed to add documentation to attributes enum")
            .variant(attributes.into())
            .add_trait(attributes.display_impl())
            .unwrap()
            .build()
            .expect("Failed to build attributes enum")
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalModule<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        InternalModule::new()
            .name(ATTRIBUTES_MODULE_NAME)
            .expect("Failed to set attributes module name")
            .public()
            .documentation(format!(
                "Submodule providing the attributes enumeration for the `{}` table.",
                attributes.table.table_name()
            ))
            .expect("Failed to add documentation to attributes module")
            .data(attributes.into())
            .expect("Failed to add attributes enum to attributes module")
            .build()
            .expect("Failed to build attributes module")
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalCrate<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(attributes.table.table_attributes_crate_name())
            .expect("Failed to set attributes crate name")
            .documentation(format!(
                "Crate containing the attributes enumeration for the `{}` table.",
                attributes.table.table_name()
            ))
            .expect("Failed to add documentation to attributes crate")
            .module(attributes.into())
            .expect("Failed to add attributes module to attributes crate")
            .build()
            .expect("Failed to build attributes crate")
    }
}
