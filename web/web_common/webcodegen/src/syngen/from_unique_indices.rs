//! Submodule implementing a `from_{x}` and `from_{x}_and_{y}` methods for each
//! Unique index associated to the Table structs.

use diesel_async::AsyncPgConnection;
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
    pub async fn from_unique_indices(
        &self,
        conn: &mut AsyncPgConnection,
        syntax: &crate::codegen::Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_name_ident = self.import_diesel_path()?;
        let primary_keys = self.primary_key_columns(conn).await?;
        let syntax_flag = syntax.as_feature_flag();
        let syntax_connection = syntax.as_connection_type(true);
        let mut unique_indices = TokenStream::new();

        for index in self.unique_indices(conn).await? {
            let columns = index.columns(conn).await?;

            let mut all_columns_are_foreign_keys = true;
            for column in &columns {
                if !column.is_foreign_key(conn).await? {
                    all_columns_are_foreign_keys = false;
                    break;
                }
            }
            if all_columns_are_foreign_keys && columns.len() == 1 {
                continue;
            }

            if columns.iter().all(|c| primary_keys.contains(c)) && primary_keys.len() == 1 {
                continue;
            }

            let method_name = format!(
                "from_{}",
                columns.iter().map(|c| c.column_name.as_str()).collect::<Vec<&str>>().join("_and_")
            );
            let method_ident = syn::Ident::new(&method_name, struct_name.span());
            let mut method_arguments = Vec::new();
            for column in &columns {
                let column_name = column.snake_case_ident()?;
                let column_type = column.rust_ref_data_type(conn).await?;
                method_arguments.push(quote! { #column_name: #column_type });
            }
            let mut include_bool_expression_methods = false;
            let filter = columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    Ok(quote! { #table_name_ident::#column_name.eq(#column_name) })
                })
                .try_fold(
                    quote! {},
                    |acc: TokenStream, filter: Result<TokenStream, WebCodeGenError>| {
                        let filter = filter?;
                        Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                            filter
                        } else {
                            include_bool_expression_methods = true;
                            quote! {#acc.and(#filter) }
                        })
                    },
                )?;

            let bool_expression_methods = if include_bool_expression_methods {
                quote! {
                    , BoolExpressionMethods
                }
            } else {
                TokenStream::new()
            };

            unique_indices.extend(quote! {
                #syntax_flag
                pub async fn #method_ident(
                    #(#method_arguments),*,
                    conn: &mut #syntax_connection
                ) -> Result<Option<Self>, diesel::result::Error> {
                    use diesel_async::RunQueryDsl;
                    use diesel::associations::HasTable;
                    use diesel::{QueryDsl, ExpressionMethods, OptionalExtension #bool_expression_methods};
                    Self::table()
                        .filter(#filter)
                        .first::<Self>(conn)
                        .await
                        .optional()
                }
            });
        }

        Ok(unique_indices)
    }
}
