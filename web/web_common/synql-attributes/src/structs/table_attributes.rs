//! Submodule defining the `TableAttributes` struct.
use quote::quote;
use syn::Ident;
use synql_core::{
    prelude::Builder,
    structs::{InternalDataVariant, InternalEnum, InternalToken},
    traits::ColumnSynLike,
};

use crate::traits::{
    TableAttributesLike, TableExtensionAttributesLike,
    table_extension_attributes_like::EXTENSION_ATTRIBUTES_ENUM_NAME,
};

mod into_crate;
mod into_data;
mod into_enum;
mod into_module;

#[derive(Debug)]
/// Struct representing a SynQL table attributes enumeration.
pub struct TableAttributes<'table, T: TableAttributesLike + ?Sized> {
    table: &'table T,
    workspace: &'table synql_core::structs::Workspace,
    database: &'table T::DB,
}

impl<'table, T: TableAttributesLike + ?Sized> Clone for TableAttributes<'table, T> {
    fn clone(&self) -> Self {
        Self { table: self.table, workspace: self.workspace, database: self.database }
    }
}

impl<'table, T: TableAttributesLike + ?Sized> Copy for TableAttributes<'table, T> {}

impl<'table, T: TableAttributesLike + ?Sized> TableAttributes<'table, T> {
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

    fn from_extension_impl(&self) -> Option<InternalToken> {
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

impl<'table, T: TableAttributesLike + ?Sized> From<TableAttributes<'table, T>>
    for InternalDataVariant
{
    fn from(attributes: TableAttributes<'table, T>) -> Self {
        let enum_variant: InternalEnum = attributes.into();
        enum_variant.into()
    }
}
