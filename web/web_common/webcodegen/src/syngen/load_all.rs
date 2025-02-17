//! Submodule implementing the `syn`-based, `diesel-async`-based `load_all`
//! method which returns all entries of a table.

use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Returns the `load_all` method for the table.
    pub fn load_all_method(&self) -> Result<TokenStream, WebCodeGenError> {
        let table_name_ident = self.snake_case_ident()?;

        Ok(quote! {
            #[cfg(feature = "diesel")]
            pub async fn load_all(conn: &mut web_common_traits::prelude::DBConn) -> Result<Vec<Self>, diesel::result::Error> {
                #table_name_ident::table.load::<Self>(conn).await
            }
        })
    }
}
