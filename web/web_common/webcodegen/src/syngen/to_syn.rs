//! Primary method to convert a Table to a struct and associated impls.

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError};

impl Table {
    async fn identifiable_impl(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        if !self.has_primary_keys(conn).await? {
            return Ok(TokenStream::new());
        }

        let struct_name: Ident = self.struct_ident()?;
        let primary_key = self.primary_key_type(conn).await?;
        let primary_key_attribute = self.primary_key_attributes(true, conn).await?;

        Ok(quote! {
            impl diesel::Identifiable for #struct_name {
                type Id = #primary_key;

                fn id(self) -> Self::Id {
                    #primary_key_attribute
                }
            }
        })
    }

    /// Returns the Syn `TokenStream` for the struct definition.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the provided connection is not valid.
    /// * If the number of columns exceeds 128.
    pub async fn to_syn(
        &self,
        conn: &mut AsyncPgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        if self.columns(conn).await?.len() > 128 {
            return Err(WebCodeGenError::ExcessiveNumberOfColumns(
                Box::new(self.clone()),
                self.columns(conn).await?.len(),
            ));
        }

        let table_path = self.import_diesel_path()?;
        let struct_name: Ident = self.struct_ident()?;

        let mut attributes = Vec::new();

        for column in self.columns(conn).await? {
            let column_attribute: Ident = column.snake_case_ident()?;
            let column_type = column.rust_data_type(conn).await?;
            attributes.push(quote! {
                pub #column_attribute: #column_type
            });
        }
        let diesel_derives_decorator = self.diesel_derives_decorator(conn).await?;
        let primary_key_decorator = self.primary_key_decorator(conn).await?;
        let mut default_derives = vec![quote!(Debug), quote!(Clone), quote!(PartialEq)];
        if self.supports_copy(conn).await? {
            default_derives.push(quote!(Copy));
        }
        if self.supports_eq(conn).await? {
            default_derives.push(quote!(Eq));
        }
        if self.supports_ord(conn).await? {
            default_derives.push(quote!(PartialOrd));
            default_derives.push(quote!(Ord));
        }
        if self.supports_hash(conn).await? {
            default_derives.push(quote!(Hash));
        }

        let identifiable_impl = self.identifiable_impl(conn).await?;

        Ok(quote! {
            #[derive(#(#default_derives),*)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[diesel(table_name = #table_path)]
            pub struct #struct_name {
                #(#attributes),*
            }

            #identifiable_impl
        })
    }
}
