use quote::quote;
use crate::errors::WebCodeGenError;
use proc_macro2::TokenStream;
use diesel::PgConnection;
use syn::Ident;
use crate::{Table, Column};

impl Table {
	/// Returns the Syn `TokenStream` for the implementation of the `InsertableVariant` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the implementation of the `InsertableVariant` trait.
    ///
    /// # Errors
    ///
    /// * If the struct name cannot be generated.
    pub fn insert_trait_impls(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let users = Table::load(conn, "users", Some(&self.table_schema), &self.table_catalog)?;
        let user_struct_name: Ident = Ident::new(&users.struct_name()?, proc_macro2::Span::call_site());
        let struct_name: Ident = Ident::new(&self.struct_name()?, proc_macro2::Span::call_site());
        let table_name = self.snake_case_ident()?;
        let columns = self.columns(conn)?;

        let session_insert_columns = columns
            .iter()
            .filter(|column| !column.is_automatically_generated())
            .collect::<Vec<&Column>>();

        let insert_columns = session_insert_columns
            .iter()
            .filter(|column| !column.is_session_user_generated(conn) && !column.is_automatically_generated())
            .collect::<Vec<&&Column>>();

        let session_insert_variant_name = Ident::new(
            &format!("SessionInsert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let session_insert_variable_attributes = session_insert_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.rust_data_type(conn)?;
                Ok(quote! {
                    pub #column_name: #column_type,
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let insert_variant_name = Ident::new(
            &format!("Insert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let insertable_variant_attributes = insert_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let column_type = column.rust_data_type(conn)?;
                Ok(quote! {
                    pub #column_name: #column_type,
                })
            })
            .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;

        let into_session_insert_variant_map = insert_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;

        let new_structs_implementation = quote! {
            #[cfg(feature = #columns_feature_flag_name)]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = #columns_feature_flag_name, derive(diesel::Insertable))]
            #[cfg_attr(feature = #columns_feature_flag_name, diesel(table_name = #table_name))]
            pub struct #session_insert_variant_name {
                #(#session_insert_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #insert_variant_name {
                #(#insertable_variant_attributes)*
            }
        };

        let updated_by_assignment: TokenStream = if self.has_updated_by_column(conn)? {
            quote! {
                updated_by: session_user.id
            }
        } else {
            TokenStream::new()
        };

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = #columns_feature_flag_name)]
            impl web_common_traits::prelude::AuthenticatedOperation for #insert_variant_name {
                type User = #user_struct_name;
                type Operation = #session_insert_variant_name;

                fn authenticate(self, session_user: &Self::User) -> Self::SessionInsertVariant {
                    #session_insert_variant_name {
                        #(#into_session_insert_variant_map)*
                        created_by: session_user.id,
                        #updated_by_assignment
                    }
                }
            }

            #[cfg(feature = #columns_feature_flag_name)]
            impl web_common_traits::prelude::Operation for #session_insert_variant_name {
                type Row = #struct_name;

                async fn insert(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<Self::Row, diesel::result::Error> {
                    diesel::insert_into(#table_name::table)
                        .values(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }
}