//! Submodule implementing a `from_{x}` and `from_{x}_and_{y}` methods for each
//! Unique index associated to the Table structs.

use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Generate the `from_{x}` and `from_{x}_and_{y}` methods for each Unique
    /// index associated to the Table structs.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    ///
    /// # Implementation details
    ///
    /// Since the foreign key methods from the [`Foreign`] trait and the `load`
    /// method from the [`Loadable`] trait cover all foreign keys and the
    /// primary keys, we only need to implement the methods relative to
    /// UNIQUE indices that do not refer to neither those columns, even if
    /// they are UNIQUE indices.
    ///
    /// # Errors
    ///
    /// * If building the table name fails.
    /// * If querying the database for the unique indices fails.
    pub fn from_unique_indices(
        &self,
        conn: &mut diesel::PgConnection,
        syntax: &crate::codegen::Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_name_ident = self.import_diesel_path()?;
        let primary_keys = self.primary_key_columns(conn)?;
        let syntax_flag = syntax.as_feature_flag();
        let syntax_connection = syntax.as_connection_type();

        self.unique_indices(conn)?
            .into_iter()
            .map(|index| {
                let columns = index.columns(conn)?;

                if columns.iter().all(|c| c.is_foreign_key(conn)) {
                    return Ok(quote! {});
                }

                if columns.iter().all(|c| primary_keys.contains(c)) {
                    return Ok(quote! {});
                }

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
                    .map(|column| {
                        let column_name = column.snake_case_ident()?;
                        let column_type = column.rust_ref_data_type(conn)?;
                        Ok(quote! { #column_name: #column_type })
                    })
                    .collect::<Result<Vec<TokenStream>, WebCodeGenError>>()?;
                let filter = columns
                    .iter()
                    .map(|column| {
                        let column_name = column.snake_case_ident()?;
                        Ok(quote! { diesel::ExpressionMethods::eq(#table_name_ident::#column_name, #column_name) })
                    })
                    .try_fold(quote! {}, |acc: TokenStream, filter: Result<TokenStream, WebCodeGenError>| {
                        let filter = filter?;
                        Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                            filter
                        } else {
                            quote! {diesel::BoolExpressionMethods::and(#acc, #filter) }
                        })
                    })?;

                Ok(quote! {
                    #syntax_flag
                    pub async fn #method_ident(
                        #(#method_arguments),*,
                        conn: &mut #syntax_connection
                    ) -> Result<Option<Self>, diesel::result::Error> {
                        use diesel_async::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::{QueryDsl, OptionalExtension};
                        Self::table()
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
