//! Primary method to convert a Table to a struct and associated impls.

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError, traits::TableLike};

impl Table {
    fn identifiable_impl(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if !self.has_primary_keys(conn)? {
            return Ok(TokenStream::new());
        }

        let struct_name: Ident = self.struct_ident()?;
        let primary_key = self.primary_key_type(conn)?;
        let primary_key_attribute = self.primary_key_attributes(true, conn)?;

        Ok(quote! {
            impl diesel::Identifiable for #struct_name {
                type Id = #primary_key;

                fn id(self) -> Self::Id {
                    #primary_key_attribute
                }
            }
        })
    }

    fn primary_key_like_impl(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<TokenStream>, WebCodeGenError> {
        if !self.has_primary_keys(conn)? {
            return Ok(None);
        }

        let struct_name: Ident = self.struct_ident()?;
        let primary_key = self.primary_key_type(conn)?;
        let primary_key_attribute = self.primary_key_attributes(true, conn)?;

        Ok(Some(quote! {
            impl web_common_traits::database::PrimaryKeyLike for #struct_name {
                type PrimaryKey = #primary_key;

                fn primary_key(&self) -> Self::PrimaryKey {
                    #primary_key_attribute
                }
            }
        }))
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
    pub fn to_syn(
        &self,
        enable_yew: bool,
        enable_insertables: bool,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        if self.columns(conn)?.len() > 128 {
            return Err(WebCodeGenError::ExcessiveNumberOfColumns(
                Box::new(self.clone()),
                self.columns(conn)?.len(),
            ));
        }

        let table_path = self.import_diesel_path()?;
        let struct_name: Ident = self.struct_ident()?;

        let mut attributes = Vec::new();

        for column in self.columns(conn)?.as_ref() {
            let column_attribute: Ident = column.snake_case_ident()?;
            let column_type = column.rust_data_type(conn)?;
            attributes.push(quote! {
                pub #column_attribute: #column_type
            });
        }
        let mut diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let primary_key_decorator = self.primary_key_decorator(conn)?;
        let mut default_derives = vec![quote!(Debug), quote!(Clone), quote!(PartialEq)];
        if self.supports_copy(conn)? {
            default_derives.push(quote!(Copy));
        }
        if self.supports_eq(conn)? {
            default_derives.push(quote!(Eq));
        }
        if self.supports_partial_ord(conn)? {
            default_derives.push(quote!(PartialOrd));
        }
        if self.supports_ord(conn)? {
            default_derives.push(quote!(Ord));
        }
        if self.supports_hash(conn)? {
            default_derives.push(quote!(Hash));
        }

        let identifiable_impl = self.identifiable_impl(conn)?;

        if enable_yew {
            diesel_derives_decorator.extend(quote! {
                #[cfg_attr(feature = "yew", derive(yew::prelude::Properties))]
            });
        }

        for singleton_foreign_key in self.singleton_foreign_keys(conn)? {
            let foreign_table = singleton_foreign_key.foreign_table(conn)?;
            let foreign_struct_path = foreign_table.import_struct_path()?;
            let column_ident = singleton_foreign_key.columns(conn)?[0].snake_case_ident()?;
            diesel_derives_decorator.extend(quote! {
                #[diesel(belongs_to(#foreign_struct_path, foreign_key = #column_ident))]
            });
        }

        let extensions_impls = self.extension_traits_impls(conn)?;
        let ancestor_impl = self.ancestor_traits_impl(conn)?;
        let primary_key_like_impl = self.primary_key_like_impl(conn)?;
        let table_name = self.table_name.clone();
        let primary_key_type = self.primary_key_type(conn)?;
        let builder_type = self.insertable_builder_ty()?;
        let primary_key_idents = self.primary_key_idents(conn)?;
        let formatted_primary_key_idents = if primary_key_idents.len() == 1 {
            quote! { value.#(#primary_key_idents)* }
        } else {
            quote! { (#(value.#primary_key_idents),*) }
        };

        let maybe_id_or_builder_from = if enable_insertables {
            Some(quote! {
                    impl<'a> From<&'a #struct_name> for web_common_traits::database::IdOrBuilder<#primary_key_type, #builder_type> {
                    fn from(value: &'a #struct_name) -> Self {
                        web_common_traits::database::IdOrBuilder::Id(#formatted_primary_key_idents)
                    }
                }
            })
        } else {
            None
        };

        Ok(quote! {
            #[derive(#(#default_derives),*)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[diesel(table_name = #table_path)]
            pub struct #struct_name {
                #(#attributes),*
            }

            impl web_common_traits::prelude::TableName for #struct_name {
                const TABLE_NAME: &'static str = #table_name;
            }

            #maybe_id_or_builder_from
            #(#extensions_impls)*
            #ancestor_impl
            #identifiable_impl
            #primary_key_like_impl
        })
    }
}
