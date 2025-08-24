//! Submodule implementing the `SetPrimaryKey` trait for the insertable
//! builder struct of a table.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use syn::Ident;
use crate::traits::TableLike;

use crate::{Codegen, Table, errors::WebCodeGenError};

impl Codegen<'_> {
    pub(super) fn generate_insertable_builder_set_primary_key(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let extension_tables = table.extension_tables(conn)?;

        let primary_key_type = table.primary_key_type(conn)?;
        let builder_ident = table.insertable_builder_ident()?;
        let extension_foreign_keys = table.extension_foreign_keys(conn)?;
        let generics = extension_tables
            .iter()
            .map(|table| table.struct_ident())
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;
        let primary_key_assignments = extension_foreign_keys
            .iter()
            .map(|foreign_key| {
                let constraint_ident = foreign_key.constraint_ident(conn)?;
                Ok(quote::quote! {
                    self.#constraint_ident = self.#constraint_ident.set_primary_key(primary_key);
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let maybe_generics =
            if generics.is_empty() { None } else { Some(quote::quote! {<#(#generics),*>}) };
        let maybe_where_statement = if generics.is_empty() {
            None
        } else {
            Some(quote::quote! {
                where
                    #(#generics: web_common_traits::prelude::SetPrimaryKey<PrimaryKey = #primary_key_type>),*
            })
        };
        let maybe_mut = if generics.is_empty() { None } else { Some(quote::quote! {mut}) };
        let primary_key_ident = if generics.is_empty() {
            quote::quote! {_primary_key}
        } else {
            quote::quote! {primary_key}
        };

        Ok(quote::quote! {
            impl #maybe_generics web_common_traits::prelude::SetPrimaryKey for #builder_ident #maybe_generics #maybe_where_statement
            {
                type PrimaryKey = #primary_key_type;

                fn set_primary_key(#maybe_mut self, #primary_key_ident: Self::PrimaryKey) -> Self {
                    #(#primary_key_assignments)*
                    self
                }
            }
        })
    }
}
