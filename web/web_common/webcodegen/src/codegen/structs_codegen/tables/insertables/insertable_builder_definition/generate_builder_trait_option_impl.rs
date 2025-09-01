//! Submodule defining the trait builder implementation for an Option
//! with as type the primary key type of the current table.
use proc_macro2::Span;
use quote::quote;
use syn::Ident;

use crate::{Table, errors::WebCodeGenError};

impl Table {
    /// Returns the trait implementation for an Option with as type the
    /// primary key type of the current table.
    ///
    /// # Arguments
    ///
    /// * `conn`: The PostgreSQL connection to use to retrieve information about
    ///   the table.
    ///
    /// # Errors
    ///
    /// * If the trait definition cannot be generated.
    /// * If the name of the variant builder cannot be retrieved.
    pub(super) fn generate_builder_trait_option_impl(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<proc_macro2::TokenStream, WebCodeGenError> {
        // If the current table has a composite primary key, we raise an unimplemented
        // as we cannot implement the trait for Option<>
        if self.has_composite_primary_key(conn)? {
            unimplemented!(
                "Insertable builder trait implementation for Option<> is not supported for tables with composite primary keys."
            );
        }

        let primary_key_type = self.primary_key_type(conn)?;
        let trait_ident = self.builder_trait_ident()?;
        let attributes = self.insertable_enum_ty()?;
        let mut method_impls = Vec::new();

        for insertable_column in self.insertable_columns(conn, false)? {
            let setter_method = insertable_column.getter_ident()?;
            let column_camel_case_ident = insertable_column.camel_case_ident()?;
            let maybe_generics = self.builder_trait_generics(&insertable_column, conn)?;
            let argument_type = self.builder_trait_argument_type(&insertable_column, conn)?;
            let maybe_where_constraints =
                self.builder_trait_where_constraints(&insertable_column, conn)?;

            let (method_body, column_snake_case_ident) =
                if insertable_column.is_primary_key(conn)? {
                    let column_snake_case_ident = insertable_column.snake_case_ident()?;
                    (
                        quote! {
                            Ok(Some(#column_snake_case_ident.try_into().map_err(
                                |err| {
                                    validation_errors::SingleFieldError::from(err)
                                        .rename_field(Self::Attributes::#column_camel_case_ident)
                                }
                            )?))
                        },
                        column_snake_case_ident.clone(),
                    )
                } else {
                    let column_snake_case_name = insertable_column.snake_case_name()?;
                    let column_snake_case_ident =
                        Ident::new(&format!("_{column_snake_case_name}"), Span::call_site());
                    (
                        quote! {
                            Ok(self)
                        },
                        column_snake_case_ident,
                    )
                };

            method_impls.push(quote! {
                fn #setter_method #maybe_generics(
                    self,
                    #column_snake_case_ident: #argument_type
                ) -> Result<
                    Self,
                    web_common_traits::database::InsertError<Self::Attributes>
                > #maybe_where_constraints
                {
                    #method_body
                }
            })
        }

        let maybe_attributes = if self.extension_tables(conn)?.is_empty() {
            Some(quote! {
                type Attributes = #attributes;
            })
        } else {
            None
        };

        Ok(quote::quote! {
            impl #trait_ident for Option<#primary_key_type> {
                #maybe_attributes

                #(#method_impls)*
            }
        })
    }
}
