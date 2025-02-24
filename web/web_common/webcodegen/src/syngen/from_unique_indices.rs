//! Submodule implementing a `from_{x}` and `from_{x}_and_{y}` methods for each
//! Unique index associated to the Table structs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Generate the `from_{x}s` methods for each Unique index associated to the
    /// Table structs which is characterized by a single column. These
    /// methods are not generated for Unique indices with multiple columns.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    fn from_single_column_unique_indices(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_name_ident = self.snake_case_ident()?;

        self.unique_indices(conn)?
            .into_iter()
            .map(|index| {
                let columns = index.columns(conn)?;
                let first_column = match columns.len() {
                    1 => columns.first().unwrap(),
                    _ => return Ok(quote! {}),
                };

                let plural_column_name = first_column.plural_column_name();
                let method_name = format!("from_{}", plural_column_name);
                let method_ident = syn::Ident::new(&method_name, struct_name.span());

                let plural_column_ident = Ident::new(&plural_column_name, struct_name.span());
                let singular_column_ident = first_column.sanitized_snake_case_ident()?;
                let column_type = first_column.rust_data_type(conn)?;

                Ok(quote! {
                    #[cfg(feature = "diesel")]
                    pub async fn #method_ident(
                        #plural_column_ident: &[#column_type],
                        conn: &mut web_common_traits::prelude::DBConn
                    ) -> Result<Vec<Self>, diesel::result::Error> {
                        #table_name_ident::table
                            .filter(#table_name_ident::#singular_column_ident.eq_any(#plural_column_ident))
                            .load::<Self>(conn)
                            .await
                    }
                })
            })
            .collect::<Result<TokenStream, WebCodeGenError>>()
    }

    /// Generate the `from_{x}` and `from_{x}_and_{y}` methods for each Unique
    /// index associated to the Table structs.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    pub fn from_unique_indices(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_name_ident = self.snake_case_ident()?;

        let single_entry_methods = self
            .unique_indices(conn)?
            .into_iter()
            .map(|index| {
                let columns = index.columns(conn)?;
                let method_name = format!(
                    "from_{}",
                    columns
                        .iter()
                        .map(|c| c.column_name.as_str())
                        .collect::<Vec<&str>>()
                        .join("_and_")
                );
                let method_ident = syn::Ident::new(&method_name, struct_name.span());
                let method_arguments = columns
                    .iter()
                    .map(|c| {
                        let column_name = Ident::new(&c.column_name, struct_name.span());
                        let column_type = c.rust_ref_data_type(conn)?;
                        Ok(quote! { #column_name: #column_type })
                    })
                    .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
                let filter = columns
                    .iter()
                    .map(|c| {
                        let column_name = Ident::new(&c.column_name, struct_name.span());
                        quote! { #table_name_ident::#column_name.eq(#column_name) }
                    })
                    .fold(quote! {}, |acc, filter| {
                        if acc.is_empty() {
                            filter
                        } else {
                            quote! {diesel::BoolExpressionMethods::and(#acc, #filter) }
                        }
                    });

                Ok(quote! {
                    #[cfg(feature = "diesel")]
                    pub async fn #method_ident(
                        #(#method_arguments),*,
                        conn: &mut web_common_traits::prelude::DBConn
                    ) -> Result<Option<Self>, diesel::result::Error> {
                        diesel_utils::optional(
                            #table_name_ident::table
                                .filter(#filter)
                                .first::<Self>(conn)
                                .await
                        )
                    }
                })
            })
            .collect::<Result<TokenStream, WebCodeGenError>>()?;

        Ok(single_entry_methods)
    }
}
