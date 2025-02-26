//! Submodule implementing a `from_{x}` and `from_{x}_and_{y}` methods for each
//! Unique index associated to the Table structs.

use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Generate the `from_{x}` and `from_{x}_and_{y}` methods for each Unique
    /// index associated to the Table structs.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    ///
    /// # Implementative details
    ///
    /// Since the foreign key methods from the [`Foreign`] trait and the `load`
    /// method from the [`Loadable`] trait cover all foreign keys and the primary
    /// keys, we only need to implement the methods relative to UNIQUE indices that
    /// do not refer to neither those columns, even if they are UNIQUE indices.
    ///
    /// # Errors
    ///
    /// * If building the table name fails.
    /// * If querying the database for the unique indices fails.
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
                        #table_name_ident::table
                            .filter(#filter)
                            .first::<Self>(conn)
                            .await
                            .optional()
                    }
                })
            })
            .collect::<Result<TokenStream, WebCodeGenError>>()
    }
}
