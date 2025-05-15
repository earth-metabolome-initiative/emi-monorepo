use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::errors::WebCodeGenError;

impl crate::Table {
    /// Generate the `from_{x}` methods for the attributes of the struct
    /// which are not a unique index or a foreign key.
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
    pub async fn from_attributes(
        &self,
        conn: &mut AsyncPgConnection,
        syntax: &crate::codegen::Syntax,
    ) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_diesel_path = self.import_diesel_path()?;
        let table_ident = self.snake_case_ident()?;
        let syntax_flag = syntax.as_feature_flag();
        let syntax_connection = syntax.as_connection_type(true);
        let mut from_attributes = TokenStream::new();
        let primary_key_order_by = self
            .primary_key_identifiers(conn)
            .await?
            .into_iter()
            .map(|column_ident| {
                quote! {
                    #table_ident::#column_ident.asc()
                }
            })
            .collect::<Vec<_>>();

        let primary_key_order_by = if primary_key_order_by.len() > 1 {
            quote! {
                (#(#primary_key_order_by),*)
            }
        } else {
            quote! {
                #(#primary_key_order_by)*
            }
        };

        for column in self.columns(conn).await? {
            // If the column is a UNIQUE index or a foreign key, skip it, as
            // the method generation is already taken care of elsewhere.
            if column.is_unique(conn).await?
                || column.is_foreign_key(conn).await?
                || !self.supports_eq(conn).await?
            {
                continue;
            }

            let column_ident = column.snake_case_ident()?;
            let column_type_ident = column.rust_data_type(conn).await?;
            let method_ident =
                syn::Ident::new(&format!("from_{}", column.snake_case_name()?), struct_name.span());

            from_attributes.extend(quote! {
                #syntax_flag
                pub async fn #method_ident(
                    #column_ident: &#column_type_ident,
                    conn: &mut #syntax_connection
                ) -> Result<Vec<Self>, diesel::result::Error> {
                    use diesel_async::RunQueryDsl;
                    use diesel::associations::HasTable;
                    use diesel::{QueryDsl, ExpressionMethods};
                    use #table_diesel_path;
                    Self::table()
                        .filter(#table_ident::#column_ident.eq(#column_ident))
                        .order_by(#primary_key_order_by)
                        .load::<Self>(conn)
                        .await
                }
            });
        }

        Ok(from_attributes)
    }
}
