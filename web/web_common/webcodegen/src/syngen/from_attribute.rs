use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Column, Table, errors::WebCodeGenError};

impl crate::Table {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::type_complexity)]
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
    pub fn from_attributes(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_ident = self.snake_case_ident()?;
        let mut from_attributes = TokenStream::new();
        let primary_key_columns = self.primary_key_columns(conn)?;
        let mut from_methods: Vec<(Vec<Table>, Vec<Column>, bool, Option<TokenStream>)> =
            Vec::new();
        let primary_key_order_by = self
            .primary_key_identifiers(conn)?
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

        for index in self.unique_indices(conn)? {
            if index.is_primary_key() {
                continue;
            }

            let columns = index.columns(conn)?;

            from_methods.push((vec![self.clone()], columns, true, None));
        }

        let mut same_as_constraints = Vec::new();

        for column in self.columns(conn)? {
            // If the column is a UNIQUE index skip it, as
            // the method generation is already taken care of elsewhere.
            if column.is_unique(conn)? || !column.supports_eq(conn)? {
                continue;
            }

            same_as_constraints.extend(column.same_as_columns(conn)?);
            from_methods.push((vec![self.clone()], vec![column], false, None));
        }

        for foreign_key_constraint in self.foreign_keys(conn)? {
            let columns = foreign_key_constraint.columns(conn)?;

            // The foreign keys which are either a single column or part
            // of a unique constraints are already covered by other methods.
            if columns.len() == 1 {
                continue;
            }

            from_methods.push((vec![self.clone()], columns, false, None));
        }

        for extension_table in self.ancestral_extension_tables(conn)? {
            let extension_table_ident = extension_table.snake_case_ident()?;
            let extension_primary_key = extension_table.primary_key_columns(conn)?.pop().unwrap();
            let primary_key_ident = primary_key_columns[0].snake_case_ident()?;
            let extension_primary_key_ident = extension_primary_key.snake_case_ident()?;

            let join_clause = quote! {
                #extension_table_ident::table.on(
                    #table_ident::#primary_key_ident
                        .eq(#extension_table_ident::#extension_primary_key_ident)
                )
            };

            for index in extension_table.unique_indices(conn)? {
                if index.is_primary_key() {
                    continue;
                }

                let columns = index.columns(conn)?;

                if columns.len() == 1 && same_as_constraints.iter().any(|c| c == &columns[0]) {
                    continue;
                }

                from_methods.push((
                    vec![self.clone(), extension_table.clone()],
                    columns,
                    true,
                    Some(join_clause.clone()),
                ));
            }

            for column in extension_table.columns(conn)? {
                // If the column is a UNIQUE index or a foreign key, skip it, as
                // the method generation is already taken care of elsewhere.
                if column.is_unique(conn)? || !column.supports_eq(conn)? {
                    continue;
                }

                // If the column appears in the same-as constraints,
                // we skip it, as the method generation is already taken care of elsewhere.
                if same_as_constraints.iter().any(|c| c == &column) {
                    continue;
                }

                from_methods.push((
                    vec![self.clone(), extension_table.clone()],
                    vec![column],
                    false,
                    Some(join_clause.clone()),
                ));
            }

            for foreign_key_constraint in extension_table.foreign_keys(conn)? {
                let columns = foreign_key_constraint.columns(conn)?;

                // The foreign keys which are either a single column or part
                // of a unique constraints are already covered by other methods.
                if columns.len() == 1 {
                    continue;
                }

                from_methods.push((
                    vec![self.clone(), extension_table.clone()],
                    columns,
                    false,
                    Some(join_clause.clone()),
                ));
            }
        }

        for (tables, columns, is_unique, join_clause) in from_methods {
            let method_ident = syn::Ident::new(
                &format!(
                    "from_{}",
                    columns
                        .iter()
                        .map(Column::snake_case_name)
                        .collect::<Result<Vec<String>, _>>()?
                        .join("_and_")
                ),
                struct_name.span(),
            );

            let mut additional_imports = tables
                .iter()
                .map(Table::import_diesel_path)
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            let ok_return_type = if is_unique {
                quote! { Self }
            } else {
                quote! { Vec<Self> }
            };

            let load_statement = if is_unique {
                quote! { .first::<Self>(conn) }
            } else {
                quote! { .load::<Self>(conn) }
            };

            let mut include_bool_expression_methods = false;

            let where_statement = columns
                .iter()
                .map(|column| {
                    let column_name = column.snake_case_ident()?;
                    let table = column.table(conn)?;
                    let table_name_ident = table.snake_case_ident()?;
                    Ok(quote! { #table_name_ident::#column_name.eq(#column_name) })
                })
                .try_fold(
                    TokenStream::new(),
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

            if include_bool_expression_methods {
                additional_imports.push(syn::parse_quote! { diesel::BoolExpressionMethods });
            }

            let arguments = columns
                .iter()
                .map(|column| {
                    let column = column.to_non_nullable();
                    let column_name = column.snake_case_ident()?;
                    let column_type = column.rust_ref_data_type(conn)?;
                    Ok(quote! { #column_name: #column_type })
                })
                .collect::<Result<Vec<_>, WebCodeGenError>>()?;

            from_attributes.extend(if let Some(join_clause) = join_clause {
                quote::quote! {
                    #[cfg(feature = "postgres")]
                    pub fn #method_ident(
                        #(#arguments,)*
                        conn: &mut diesel::PgConnection,
                    ) -> Result<#ok_return_type, diesel::result::Error> {
                        use diesel::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::{QueryDsl, ExpressionMethods, SelectableHelper, JoinOnDsl};
                        #(use #additional_imports;)*
                        Self::table()
                            .inner_join(#join_clause)
                            .filter(#where_statement)
                            .order_by(#primary_key_order_by)
                            .select(Self::as_select())
                            #load_statement
                    }
                }
            } else {
                quote! {
                    #[cfg(feature = "postgres")]
                    pub fn #method_ident(
                        #(#arguments,)*
                        conn: &mut diesel::PgConnection,
                    ) -> Result<#ok_return_type, diesel::result::Error> {
                        use diesel::RunQueryDsl;
                        use diesel::associations::HasTable;
                        use diesel::{QueryDsl, ExpressionMethods};
                        #(use #additional_imports;)*
                        Self::table()
                            .filter(#where_statement)
                            .order_by(#primary_key_order_by)
                            #load_statement
                    }
                }
            });
        }

        Ok(from_attributes)
    }
}
