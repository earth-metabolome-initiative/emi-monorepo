use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::{errors::WebCodeGenError, Column, Table};

impl Table {
    /// Returns the primary key columns for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn update_trait_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        if !self.has_updated_by_column(conn)? {
            return Err(WebCodeGenError::MissingUpdatedByColumn(Box::new(self.clone())));
        }

        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let table_name = self.snake_case_ident()?;
        let columns = self.columns(conn)?;
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| column.column_name.clone())
            .collect::<Vec<String>>();

        // We compose the filter expression based on the primary key columns.
        let filter_expression = primary_key_names
            .iter()
            .map(|column_name| {
                let column_name: Ident = Ident::new(column_name, proc_macro2::Span::call_site());
                quote! {
                    #table_name::#column_name.eq(&self.#column_name)
                }
            })
            .reduce(|a, b| quote! { diesel::BoolExpressionMethods::and(#a, #b)})
            .unwrap();

        let session_update_columns = columns
            .iter()
            .filter(|column| {
                (!column.is_automatically_generated()
                    || primary_key_names.contains(&column.column_name))
                    && !column.is_created_by(conn)
            })
            .collect::<Vec<&Column>>();

        let update_columns = session_update_columns
            .iter()
            .filter(|column| {
                !column.is_session_user_generated(conn)
                    || primary_key_names.contains(&column.column_name)
            })
            .collect::<Vec<&&Column>>();

        let session_update_variant_name =
            Ident::new(&format!("SessionUpdate{struct_name}"), proc_macro2::Span::call_site());
        let session_update_variable_attributes = session_update_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.rust_data_type(conn)?;
                Ok(
                    if column.is_updated_by(conn) || primary_key_names.contains(&column.column_name)
                    {
                        quote! {
                            pub #column_name: #column_type,
                        }
                    } else {
                        quote! {
                            pub #column_name: Option<#column_type>,
                        }
                    },
                )
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let update_variant_name =
            Ident::new(&format!("Update{struct_name}"), proc_macro2::Span::call_site());
        let updateable_variant_attributes = update_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.rust_data_type(conn)?;

                Ok(if primary_key_names.contains(&column.column_name) {
                    quote! {
                        pub #column_name: #column_type,
                    }
                } else {
                    quote! {
                        pub #column_name: Option<#column_type>,
                    }
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let into_session_update_variant_map = update_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        // In some cases, the table will not have a primary key. In which case, we
        // cannot specify the primary key decorator on the struct.
        let primary_key_decorator = self.primary_key_decorator(conn)?;
        let diesel_derives_decorator = self.diesel_derives_decorator(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let new_structs_implementation = quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = #columns_feature_flag_name, derive(diesel::AsChangeset))]
            #diesel_derives_decorator
            #primary_key_decorator
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #table_name))]
            pub struct #session_update_variant_name {
                #(#session_update_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #update_variant_name {
                #(#updateable_variant_attributes)*
            }
        };

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = #columns_feature_flag_name)]
            impl IntoSessionUpdateVariant for #update_variant_name {
                type SessionUpdateVariant = #session_update_variant_name;

                fn into_session_update_variant(self, updated_by: i32) -> Self::SessionUpdateVariant {
                    #session_update_variant_name {
                        #(#into_session_update_variant_map)*
                        updated_by
                    }
                }
            }

            #[cfg(feature = #columns_feature_flag_name)]
            impl UpdateableVariant<web_common_traits::prelude::DBConn> for #session_update_variant_name {
                type Row = #struct_name;

                async fn update(&self, conn: &mut web_common_traits::prelude::DBConn) -> Result<Self::Row, diesel::result::Error> {
                    diesel::update(#table_name::table)
                        .filter(#filter_expression)
                        .set(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }
}
