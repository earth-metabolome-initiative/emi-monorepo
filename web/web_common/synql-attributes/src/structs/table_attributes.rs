//! Submodule defining the `TableAttributes` struct.
use std::borrow::Borrow;

use quote::quote;
use sql_traits::traits::ForeignKeyLike;
use syn::Ident;
use synql_core::{
    prelude::Builder,
    structs::{InternalDataVariant, InternalEnum, InternalToken, TraitVariantRef},
    traits::ColumnSynLike,
    utils::generic_type,
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

    /// Implements the `From` traits for conversions of extension attributes
    /// or column attributes which are uncertain i.e. there is no multiple
    /// options of possible sources.
    ///
    /// # Implementation details
    ///
    /// * If an extended table does not appear as a possible target of a column
    ///   foreign key, then it is possible to convert from the column attribute
    ///   directly to the extended table attributes.
    /// * If a table which is NOT an extended table appears as a possible target
    ///   of a column singleton foreign key, i.e. a foreign key which is the
    ///   only foreign key referencing its target table, then it is possible to
    ///   convert from the column attribute directly to the target table
    ///   attributes.
    fn from_impls(&self) -> Vec<InternalToken> {
        let mut from_impls = Vec::new();

        let from_trait =
            self.workspace.external_trait("From").expect("Core crate must have From trait");

        for extended_table in self.table.extended_tables(self.database) {
            // If there exists a foreign key from this table to the extended table
            // which is not a host primary key reference, then we cannot have a
            // direct conversion from the column attribute to the extended table
            // attributes.
            if self.table.foreign_keys(self.database).any(|fk| {
                let referenced_table: &T = fk.referenced_table(self.database).borrow();
                !fk.is_host_primary_key(self.database)
                    && !fk.is_composite(self.database)
                    && referenced_table == extended_table
            }) {
                continue;
            }

            let extended_table_attributes = extended_table.attributes_ref(self.workspace).unwrap();
            let enum_ident = self.table.table_attributes_ident();

            let from_trait: TraitVariantRef = from_trait
                .set_generic_field(&generic_type("T"), extended_table_attributes.clone().into())
                .unwrap()
                .into();

            let from_trait_with_generic = from_trait.format_with_generics();

            from_impls.push(
                InternalToken::new()
                    .private()
                    .stream(quote! {
                        impl #from_trait_with_generic for #enum_ident {
                            fn from(attrs: #extended_table_attributes) -> Self {
                                Self::Extension(attrs.into())
                            }
                        }
                    })
                    .implemented_trait(from_trait)
                    .build()
                    .unwrap(),
            );
        }

        from_impls
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
