//! Submodule defining the `TableExtensionAttributes` struct.
use quote::quote;
use synql_core::{
    prelude::Builder,
    structs::{
        InternalCrate, InternalData, InternalDataVariant, InternalEnum, InternalModule,
        InternalToken, InternalVariant,
    },
};

use crate::traits::{
    TableAttributesLike, TableExtensionAttributesLike,
    table_extension_attributes_like::EXTENSION_ATTRIBUTES_MODULE_NAME,
};

#[derive(Debug)]
/// Struct representing a SynQL table extension attributes enumeration.
///
/// This struct is used to generate the extension attributes enum for a table
/// that extends other tables. The extension attributes enum contains variants
/// for each extended table's attributes.
pub struct TableExtensionAttributes<'data, 'table, T: TableExtensionAttributesLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace<'data>,
    database: &'table T::DB,
}

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized> Clone
    for TableExtensionAttributes<'data, 'table, T>
{
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'data, 'table, T: TableExtensionAttributesLike + TableAttributesLike + ?Sized>
    TableExtensionAttributes<'data, 'table, T>
{
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
        let enum_ident = self.table.table_extension_attributes_ident();
        let table_singular_camel_name = self.table.table_singular_camel_name();

        let variants =
            self.table.extended_tables(self.database).into_iter().map(|extended_table| {
                let extended_table_camel_ident = extended_table.table_singular_camel_ident();
                let display_pattern = format!("{table_singular_camel_name}({{e}})");
                quote! {
                    Self::#extended_table_camel_ident(e) => write!(f, #display_pattern),
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

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalEnum<'data>
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        InternalEnum::new()
            .variants(extension_attributes.table.extended_tables(extension_attributes.database).into_iter().map(|extended_table| {
				let attribute_enum = extended_table.attributes_ref(extension_attributes.workspace).expect(&format!(
					"Failed to retrieve the attributes enum for extended table `{}` from table `{}`",
					extended_table.table_name(),
					extension_attributes.table.table_name()
				));
				let extended_table_camel_ident = extended_table.table_singular_camel_ident();
                InternalVariant::new()
                    .name(extended_table_camel_ident)
                    .doc(format!(
                        "Extension attribute variant for the `{}` table, containing {} from the extended `{}` table.",
                        extension_attributes.table.table_name(),
						attribute_enum.documentation_path(),
                        extended_table.table_name()
                    ))
                    .expect("Failed to set documentation for extension attribute variant")
                    .ty(attribute_enum)
                    .build()
                    .expect("Failed to build extension attribute variant")
            }))
            .expect("Failed to set variants for extension attributes enum")
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalDataVariant<'data>
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        let enum_variant: InternalEnum<'data> = extension_attributes.into();
        enum_variant.into()
    }
}

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalData<'data>
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        let display_impl = extension_attributes.display_impl();
        let extension_attributes_clone = extension_attributes.clone();

        InternalData::new()
            .name(extension_attributes.table.table_extension_attributes_name())
            .expect("Failed to set extension attributes enum name")
            .public()
            .documentation(format!(
                "Enumeration of the extension attributes of the `{}` table.",
                extension_attributes.table.table_name()
            ))
            .expect("Failed to add documentation to extension attributes enum")
            .variant(extension_attributes_clone.into())
            .add_trait(display_impl)
            .unwrap()
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalModule<'data>
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        InternalModule::new()
            .name(EXTENSION_ATTRIBUTES_MODULE_NAME)
            .expect("Failed to set extension attributes module name")
            .public()
            .documentation(format!(
                "Submodule providing the extension attributes enumeration for the `{}` table.",
                extension_attributes.table.table_name()
            ))
            .expect("Failed to add documentation to extension attributes module")
            .data(extension_attributes.into())
            .expect("Failed to add extension attributes enum to extension attributes module")
            .build()
            .unwrap()
    }
}

impl<'data, 'table, T: TableExtensionAttributesLike + ?Sized>
    From<TableExtensionAttributes<'data, 'table, T>> for InternalCrate<'data>
{
    fn from(extension_attributes: TableExtensionAttributes<'data, 'table, T>) -> Self {
        InternalCrate::new()
            .name(extension_attributes.table.table_extension_attributes_crate_name())
            .expect("Failed to set extension attributes crate name")
            .documentation(format!(
                "Crate containing the extension attributes enumeration for the `{}` table.",
                extension_attributes.table.table_name()
            ))
            .expect("Failed to add documentation to extension attributes crate")
            .module(extension_attributes.into())
            .expect("Failed to add extension attributes module to extension attributes crate")
            .build()
            .unwrap()
    }
}
