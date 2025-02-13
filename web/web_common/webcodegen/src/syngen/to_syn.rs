//! Primary method to convert a Table to a struct and associated impls.

use diesel::PgConnection;
use syn::Ident;
use proc_macro2::TokenStream;
use quote::quote;
use crate::errors::WebCodeGenError;
use crate::Table;

impl Table {

    /// Returns the Syn `TokenStream` for the struct definition and associated methods.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the struct definition and associated methods.
    ///
    /// # Errors
    ///
    /// * If the number of columns exceeds 128.
    ///
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        if self.columns(conn)?.len() > 128 {
            return Err(WebCodeGenError::ExcessiveNumberOfColumns(
                Box::new(self.clone()),
                self.columns(conn)?.len(),
            ));
        }

        let sanitized_table_name = self.snake_case_ident()?;
        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let attributes = self
            .columns(conn)?
            .into_iter()
            .map(|column| {
                let column_attribute: Ident = column.sanitized_snake_case_ident()?;
                let column_type = column.rust_data_type(conn)?;
                Ok(quote! {
                    pub #column_attribute: #column_type
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let mutability_impls = if self.has_session_user_generated_columns(conn)? {
            let insert_trait_impls = self.insert_trait_impls(conn)?;
            let update_trait_impls = if self.has_updated_by_column(conn)? {
                self.update_trait_impls(conn)?
            } else {
                TokenStream::new()
            };
            quote! {
                #insert_trait_impls
                #update_trait_impls
            }
        } else {
            TokenStream::new()
        };
        let foreign_key_methods = self.foreign_key_methods(conn)?;
        // We only create a delete method if the table has a created_by column, which
        // means it contains user-generated data and therefore things that can be deleted
        // by some user.
        let delete_method = if self.has_created_by_column(conn)? {
            self.delete_method(conn)?
        } else {
            TokenStream::new()
        };
        let primary_key_decorator = self.primary_key_decorator(conn)?;
        let diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let built_table_syn = quote! {
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #sanitized_table_name))]
            pub struct #struct_name {
                #(#attributes),*
            }

            impl #struct_name {
                #(#foreign_key_methods)*
                #delete_method
            }

            #mutability_impls
        };

        // Convert the generated TokenStream to a string
        let code_string = built_table_syn.to_string();

        // Parse the generated code string into a syn::Item
        if let Err(error) = syn::parse_str::<syn::File>(&code_string) {
            return Err(WebCodeGenError::IllegalTableCodegen(
                format!("Error building table struct: {error}"),
                code_string,
                Box::new(self.clone()),
            ));
        }

        Ok(built_table_syn)
    }
}