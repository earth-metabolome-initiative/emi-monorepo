//! Submodule implementing a `from_{x}` and `from_{x}_and_{y}` methods for each Unique
//! index associated to the Table structs.

use crate::errors::WebCodeGenError;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

impl crate::Table {
    /// Generate the `from_{x}` and `from_{x}_and_{y}` methods for each Unique index associated to the Table structs.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    ///
    pub fn from_unique_indices(
        &self,
        conn: &mut diesel::PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_name_ident = self.snake_case_ident()?;

        self.unique_indices(conn)?
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
                let method_name = syn::Ident::new(&method_name, struct_name.span());
                let method_arguments = columns
                    .iter()
                    .map(|c| {
                        let column_name = Ident::new(&c.column_name, struct_name.span());
                        let column_type = c.rust_data_type(conn)?;
                        Ok(quote! { #column_name: &#column_type })
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
                    pub async fn #method_name(
                        #(#method_arguments),*,
                        conn: &mut diesel_async::AsyncPgConnection
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
            .collect::<Result<TokenStream, WebCodeGenError>>()
    }
}
