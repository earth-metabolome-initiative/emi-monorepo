//! Submodule providing the code to generate the implementation of the
//! [`Upsertable`](web_common_traits::prelude::Upsertable) trait for all
//! requiring tables.

use std::path::Path;

use diesel_async::AsyncPgConnection;
use proc_macro2::TokenStream;
use quote::quote;
use strum::IntoEnumIterator;

use crate::{Codegen, Table, codegen::Syntax, errors::WebCodeGenError};

impl Codegen<'_> {
    /// Generates the [`Upsertable`](web_common_traits::prelude::Upsertable)
    /// trait implementation for the tables
    ///
    /// # Arguments
    ///
    /// * `root` - The root path for the generated code.
    /// * `tables` - The list of tables for which to generate the diesel code.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    /// * If the file system fails.
    pub(super) async fn generate_upsertables_impls(
        &self,
        root: &Path,
        tables: &[Table],
        conn: &mut AsyncPgConnection,
    ) -> Result<(), crate::errors::WebCodeGenError> {
        std::fs::create_dir_all(root)?;

        // We generate each table in a separate document under the provided root, and we
        // collect all of the imported modules in a public one.
        let mut table_upsertable_main_module = TokenStream::new();
        let run_query_dsl = Syntax::as_run_query_dsl(false);

        for table in tables {
            let table_struct = table.import_struct_path()?;
            let snake_case_ident = table.snake_case_ident()?;
            // We create a file for each table
            let table_file = root.join(format!("{}.rs", table.snake_case_name()?));
            let diesel_table_path = table.import_diesel_path()?;

            let primary_key_columns: Vec<TokenStream> = table
                .primary_key_columns(conn)
                .await?
                .into_iter()
                .map(|primary_key| {
                    let snake_case_column_ident = primary_key.snake_case_ident()?;
                    Ok(quote! {
                        #snake_case_column_ident
                    })
                })
                .collect::<Result<_, WebCodeGenError>>()?;

            let formatted_primary_key_columns = match primary_key_columns.len() {
                0 => unreachable!("Table {} has no primary key columns", table.table_name),
                1 => primary_key_columns[0].clone(),
                _ => {
                    quote! {
                        (#(#primary_key_columns),*)
                    }
                }
            };

            let non_primary_key_columns = table.non_primary_key_columns(conn).await?;
            let mut include_bool_expression_methods = false;

            let conflict_clause: TokenStream = if non_primary_key_columns.is_empty() {
                quote! {
                    .do_nothing()
                }
            } else {
                let filter_ops: TokenStream = non_primary_key_columns
                    .iter()
                    .map(|column| {
                        let snake_case_column_ident = column.snake_case_ident()?;
                        Ok(quote! {
                            #snake_case_column_ident.ne(
                                excluded(#snake_case_column_ident)
                            )
                        })
                    })
                    .try_fold(
                        quote! {},
                        |acc: TokenStream, filter: Result<TokenStream, WebCodeGenError>| {
                            let filter = filter?;
                            Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                                filter
                            } else {
                                include_bool_expression_methods = true;
                                quote! {#acc.or(#filter) }
                            })
                        },
                    )?;

                quote! {
                    .do_update()
                    .set(self)
                    .filter(#filter_ops)
                }
            };

            let mut expression_methods = if non_primary_key_columns.is_empty() {
                TokenStream::new()
            } else {
                quote! {
                    use diesel::ExpressionMethods;
                    use diesel::query_dsl::methods::FilterDsl;
                    use diesel::upsert::excluded;
                }
            };

            if include_bool_expression_methods {
                expression_methods.extend(quote! {
                    use diesel::BoolExpressionMethods;
                });
            }

            // impl Deletable for struct_ident
            std::fs::write(&table_file, self.beautify_code(&Syntax::iter().map(|syntax| {
				let feature_flag = syntax.as_feature_flag();
				let connection_type = syntax.as_connection_type(false);
				quote! {
				#feature_flag
                impl web_common_traits::prelude::Upsertable<#connection_type> for #table_struct{
                    fn upsert(
						&self,
						conn: &mut #connection_type,
					) -> Result<Option<Self>, diesel::result::Error> {
						#expression_methods
						use #run_query_dsl;
                        use #diesel_table_path::*;
						diesel::insert_into(table)
							.values(self)
							.on_conflict(#formatted_primary_key_columns)
							#conflict_clause
							.get_results(conn)
							.map(|mut result| {
								result.pop()
							})
					}
                }
            }}).collect::<TokenStream>())?)?;

            table_upsertable_main_module.extend(quote::quote! {
                mod #snake_case_ident;
            });
        }

        let table_module = root.with_extension("rs");
        std::fs::write(&table_module, self.beautify_code(&table_upsertable_main_module)?)?;

        Ok(())
    }
}
