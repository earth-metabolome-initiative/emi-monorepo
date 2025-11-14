//! Submodule defining the `TableExtensionAttributes` struct.
use quote::quote;
use synql_core::{
    prelude::Builder,
    structs::{InternalDataVariant, InternalEnum, InternalToken, TraitVariantRef},
    utils::generic_type,
};

use crate::traits::{TableAttributesLike, TableExtensionAttributesLike};

mod into_crate;
mod into_data;
mod into_enum;
mod into_module;

#[derive(Debug)]
/// Struct representing a SynQL table extension attributes enumeration.
///
/// This struct is used to generate the extension attributes enum for a table
/// that extends other tables. The extension attributes enum contains variants
/// for each extended table's attributes.
pub struct TableExtensionAttributes<'table, T: TableExtensionAttributesLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableExtensionAttributesLike + ?Sized> Clone
    for TableExtensionAttributes<'table, T>
{
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableExtensionAttributesLike + TableAttributesLike + ?Sized>
    TableExtensionAttributes<'table, T>
{
    pub(crate) fn new(
        table: &'table T,
        workspace: &'table synql_core::structs::Workspace,
        database: &'table T::DB,
    ) -> Self {
        Self { table, workspace, database }
    }

    fn display_impl(&self) -> InternalToken {
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
            .implemented_trait(display_trait.clone().into())
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

    fn from_impls(&self) -> Vec<InternalToken> {
        let enum_ident = self.table.table_extension_attributes_ident();

        let from_trait =
            self.workspace.external_trait("From").expect("Core crate must have From trait");

        self.table
            .extended_tables(self.database)
            .into_iter()
            .map(|extended_table| {
                let extended_table_camel_ident = extended_table.table_singular_camel_ident();
                let extended_table_attributes =
                    extended_table.attributes_ref(self.workspace).unwrap();

                let attr_from_trait: TraitVariantRef = from_trait
                    .set_generic_field(&generic_type("T"), extended_table_attributes.clone().into())
                    .unwrap()
                    .into();

                let attr_from_trait_with_generic = attr_from_trait.format_with_generics();

                InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl #attr_from_trait_with_generic for #enum_ident {
                            fn from(attrs: #extended_table_attributes) -> Self {
                                Self::#extended_table_camel_ident(attrs)
                            }
                        }
                    })
                    .implemented_trait(attr_from_trait)
                    .build()
                    .unwrap()
            })
            .collect()
    }
}

impl<'table, T: TableExtensionAttributesLike + ?Sized> From<TableExtensionAttributes<'table, T>>
    for InternalDataVariant
{
    fn from(extension_attributes: TableExtensionAttributes<'table, T>) -> Self {
        let enum_variant: InternalEnum = extension_attributes.into();
        enum_variant.into()
    }
}
