//! Submodule defining the `TableAttributes` struct.
use quote::quote;
use syn::Ident;
use synql_core::{
    prelude::Builder,
    structs::{
        Documentation, InternalCrate, InternalData, InternalDataVariant, InternalEnum,
        InternalModule, InternalToken, InternalVariant,
    },
    traits::ColumnSynLike,
};
use synql_diesel_schema::traits::ColumnSchema;

use crate::traits::{
    TableAttributesLike, TableExtensionAttributesLike,
    table_attributes_like::ATTRIBUTES_MODULE_NAME,
    table_extension_attributes_like::EXTENSION_ATTRIBUTES_ENUM_NAME,
};

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
        let table_snake_name = self.table.table_snake_name();

        let maybe_extension_variant = if self.table.is_extension(self.database) {
            let extension_camel_ident =
                Ident::new(EXTENSION_ATTRIBUTES_ENUM_NAME, proc_macro2::Span::call_site());
            Some(quote! {
                Self::#extension_camel_ident(e) => write!(f, "{e}"),
            })
        } else {
            None
        };

        let variants = self
            .table
            .columns(self.database)
            .map(|column| {
                let column_camel_ident = column.column_camel_ident();
                let column_snake_ident = column.column_snake_name();
                let display_name = format!("{}.{}", table_snake_name, column_snake_ident);
                quote! {
                    Self::#column_camel_ident => write!(f, #display_name),
                }
            })
            .chain(maybe_extension_variant);
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

    fn from_extension_impl(&self) -> Option<InternalToken<'data>> {
        let extension_attributes = self.table.extension_attributes_enum_ref(self.workspace)?;

        let enum_ident = self.table.table_attributes_ident();
        let extension_ident =
            Ident::new(EXTENSION_ATTRIBUTES_ENUM_NAME, proc_macro2::Span::call_site());

        Some(
            InternalToken::new()
                .private()
                .stream(quote! {
                    impl From<#extension_attributes> for #enum_ident {
                        fn from(extension: #extension_attributes) -> Self {
                            Self::#extension_ident(extension)
                        }
                    }
                })
                .build()
                .unwrap(),
        )
    }
}

impl<'data, 'table, T: TableAttributesLike + TableExtensionAttributesLike + ?Sized>
    From<TableAttributes<'data, 'table, T>> for InternalEnum<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        let maybe_extension_variant = if let Some(extension_attributes) =
            attributes.table.extension_attributes_enum_ref(attributes.workspace)
        {
            let extension_ident =
                Ident::new(EXTENSION_ATTRIBUTES_ENUM_NAME, proc_macro2::Span::call_site());
            Some(
                InternalVariant::new()
                    .name(extension_ident)
                    .doc(
                        Documentation::new()
                            .documentation("Extension attributes variant.")
                            .expect("Failed to set documentation for extension attribute variant")
                            .build()
                            .expect(
                                "Failed to build documentation for extension attribute variant",
                            ),
                    )
                    .ty(extension_attributes)
                    .build()
                    .expect("Failed to build extension attribute variant"),
            )
        } else {
            None
        };

        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");

        InternalEnum::new()
            .variants(attributes.table.columns(attributes.database).map(|column| {
                InternalVariant::new()
                    .name(column.column_camel_ident())
                    .doc(
                        Documentation::new()
                            .documentation(format!(
                                "Attribute representing the {} column.",
                                column.column_schema_doc_path(attributes.database)
                            ))
                            .expect("Failed to set documentation for attribute variant")
                            .internal_dependency(schema_crate_ref.clone())
                            .unwrap()
                            .build()
                            .expect("Failed to build documentation for attribute variant"),
                    )
                    .build()
                    .expect("Failed to build attribute variant")
            }))
            .expect("Failed to set variants for attributes enum")
            .variants(maybe_extension_variant)
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
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalData::new()
            .name(attributes.table.table_attributes_name())
            .expect("Failed to set attributes enum name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Enumeration of the attributes of the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .variant(attributes.into())
            .add_trait(attributes.display_impl())
            .unwrap()
            .add_traits(attributes.from_extension_impl())
            .unwrap()
            .build()
            .expect("Failed to build attributes enum")
    }
}

impl<'data, 'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'data, 'table, T>>
    for InternalModule<'data>
{
    fn from(attributes: TableAttributes<'data, 'table, T>) -> Self {
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalModule::new()
            .name(ATTRIBUTES_MODULE_NAME)
            .expect("Failed to set attributes module name")
            .public()
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Submodule providing the attributes enumeration for the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
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
        let schema_crate_ref = attributes
            .table
            .table_schema_ref(attributes.workspace)
            .expect("Failed to get table schema crate ref for attributes enum");
        InternalCrate::new()
            .name(attributes.table.table_attributes_crate_name())
            .expect("Failed to set attributes crate name")
            .documentation(
                Documentation::new()
                    .documentation(format!(
                        "Crate containing the attributes enumeration for the {} table.",
                        attributes.table.table_schema_doc_path()
                    ))
                    .unwrap()
                    .internal_dependency(schema_crate_ref)
                    .unwrap()
                    .build()
                    .unwrap(),
            )
            .module(attributes.into())
            .expect("Failed to add attributes module to attributes crate")
            .build()
            .expect("Failed to build attributes crate")
    }
}
