use std::sync::Arc;

use diesel::PgConnection;
use proc_macro2::TokenStream;
use quote::quote;

use crate::{Column, Table, errors::WebCodeGenError, traits::TableLike};

type FromMethods = Vec<(Vec<Arc<Table>>, Arc<Vec<Column>>, bool, Option<TokenStream>)>;

impl Table {
    fn ancestral_from_attributes(
        &self,
        maybe_most_concrete_column: Option<&Column>,
        root_table: &Arc<Table>,
        conn: &mut PgConnection,
    ) -> Result<FromMethods, WebCodeGenError> {
        let mut ancestral_from_attributes: FromMethods = Vec::new();
        let extension_tables = self.extension_tables(conn)?;

        let primary_key_idents = root_table.primary_key_idents(conn)?;
        let root_table_ident = root_table.snake_case_ident()?;

        // First, we process the first-level extension tables to cover
        for extension_table in extension_tables.as_ref() {
            let extension_table_ident = extension_table.snake_case_ident()?;
            let extension_primary_key_idents = extension_table.primary_key_idents(conn)?;

            let join_clause =
                primary_key_idents.iter().zip(extension_primary_key_idents.iter()).try_fold(
                    TokenStream::new(),
                    |acc: TokenStream, (root_primary_key_ident, extension_primary_key_ident)| {
                        Ok::<TokenStream, WebCodeGenError>(if acc.is_empty() {
                            quote! {
                                #root_table_ident::#root_primary_key_ident
                                    .eq(#extension_table_ident::#extension_primary_key_ident)
                            }
                        } else {
                            quote! {
                                #acc.and(
                                    #root_table_ident::#root_primary_key_ident
                                        .eq(#extension_table_ident::#extension_primary_key_ident)
                                )
                            }
                        })
                    },
                )?;

            let join_clause = quote! {
                #extension_table_ident::table.on(
                    #join_clause
                )
            };

            for index in extension_table.indices(conn)? {
                if index.is_primary_key() || index.is_same_as(conn)? {
                    continue;
                }

                let columns = index.columns(conn)?;

                if columns.is_empty() {
                    continue;
                }

                ancestral_from_attributes.push((
                    vec![root_table.clone(), extension_table.clone()],
                    columns,
                    index.is_unique(),
                    Some(join_clause.clone()),
                ));
            }

            for foreign_key_constraint in extension_table.foreign_keys(conn)?.as_ref() {
                let columns = foreign_key_constraint.columns(conn)?;

                // The foreign keys which are either a single column or part
                // of a unique constraints are already covered by other methods.
                if columns.len() == 1
                    || foreign_key_constraint.includes_local_primary_key(conn)?
                    || foreign_key_constraint.is_foreign_unique_key(conn)?.is_some()
                {
                    continue;
                }

                ancestral_from_attributes.push((
                    vec![root_table.clone(), extension_table.clone()],
                    columns,
                    false,
                    Some(join_clause.clone()),
                ));
            }

            for column in extension_table.columns(conn)?.as_ref() {
                // If the column is a UNIQUE index or a foreign key, skip it, as
                // the method generation is already taken care of elsewhere.
                if column.is_unique(conn)?
                    || !column.supports_eq(conn)?
                    || column.has_datetime_type()
                    || maybe_most_concrete_column == Some(column)
                {
                    continue;
                }

                ancestral_from_attributes.push((
                    vec![root_table.clone(), extension_table.clone()],
                    Arc::new(vec![column.clone()]),
                    false,
                    Some(join_clause.clone()),
                ));
            }
        }

        // Secondly, we recursively call this method on the extension tables
        // to cover the higher-level extension tables.
        for extension_table in extension_tables.as_ref() {
            ancestral_from_attributes.extend(extension_table.ancestral_from_attributes(
                maybe_most_concrete_column,
                root_table,
                conn,
            )?);
        }

        Ok(ancestral_from_attributes)
    }

    #[allow(clippy::too_many_lines)]
    #[allow(clippy::type_complexity)]
    /// Generate the `from_{x}` methods for the attributes of the struct
    /// which are not a unique index or a foreign key.
    ///
    /// # Arguments
    ///
    /// * `conn` - The Diesel connection to the database.
    ///
    /// # Errors
    ///
    /// * If building the table name fails.
    /// * If querying the database for the unique indices fails.
    pub fn from_attributes(&self, conn: &mut PgConnection) -> Result<TokenStream, WebCodeGenError> {
        let struct_name = self.struct_ident()?;
        let table_diesel_path = self.import_diesel_path()?;
        let table_ident = self.snake_case_ident()?;
        let mut from_attributes = TokenStream::new();
        let mut from_methods: FromMethods = Vec::new();
        let arc_table = Arc::new(self.clone());
        let maybe_most_concrete_column = self.most_concrete_table_column(true, conn)?;
        let primary_key_order_by = self
            .primary_key_idents(conn)?
            .into_iter()
            .map(|column_ident| {
                quote! {
                    #table_ident::#column_ident.asc()
                }
            })
            .collect::<Vec<_>>();
        let primary_key_order_by_type = self
            .primary_key_idents(conn)?
            .into_iter()
            .map(|column_ident| {
                quote! {
                    diesel::helper_types::Asc<#table_diesel_path::#column_ident>
                }
            })
            .collect::<Vec<_>>();
        let formatted_primary_key_order_by_type = if primary_key_order_by_type.len() > 1 {
            quote! {
                (#(#primary_key_order_by_type),*)
            }
        } else {
            quote! {
                #(#primary_key_order_by_type)*
            }
        };

        let primary_key_order_by = if primary_key_order_by.len() > 1 {
            quote! {
                (#(#primary_key_order_by),*)
            }
        } else {
            quote! {
                #(#primary_key_order_by)*
            }
        };

        for index in self.indices(conn)? {
            if index.is_primary_key() || index.is_same_as(conn)? {
                continue;
            }

            let columns = index.columns(conn)?;

            if columns.is_empty() {
                continue;
            }

            from_methods.push((vec![arc_table.clone()], columns, index.is_unique(), None));
        }

        for foreign_key_constraint in self.foreign_keys(conn)?.as_ref() {
            // The foreign keys which are either a single column or part
            // of a unique constraints are already covered by other methods.
            if foreign_key_constraint.is_singleton(conn)?
                || foreign_key_constraint.includes_local_primary_key(conn)?
            {
                continue;
            }

            let columns = foreign_key_constraint.columns(conn)?;

            from_methods.push((vec![arc_table.clone()], columns, false, None));
        }

        for column in self.columns(conn)?.as_ref() {
            // If the column is a UNIQUE index skip it, as
            // the method generation is already taken care of elsewhere.
            if column.is_unique(conn)?
                || !column.supports_eq(conn)?
                || column.has_datetime_type()
                || column.has_singleton_foreign_key(conn)?
                || Some(column) == maybe_most_concrete_column.as_ref()
            {
                continue;
            }

            from_methods.push((
                vec![arc_table.clone()],
                Arc::new(vec![column.clone()]),
                false,
                None,
            ));
        }

        let extension_tables = self.ancestral_extension_tables(conn)?;

        if !extension_tables.is_empty() {
            from_methods.extend(self.ancestral_from_attributes(
                maybe_most_concrete_column.as_ref(),
                &arc_table,
                conn,
            )?);
        }

        let mut method_names = std::collections::HashSet::new();

        for (tables, columns, is_unique, join_clause) in from_methods {
            let method_name = &format!(
                "from_{}",
                columns
                    .iter()
                    .map(Column::snake_case_name)
                    .collect::<Result<Vec<String>, _>>()?
                    .join("_and_")
            );

            if !method_names.insert(method_name.clone()) {
                continue;
            }

            let method_ident = syn::Ident::new(method_name, struct_name.span());

            let mut additional_imports = tables
                .iter()
                .map(|table| table.import_diesel_path())
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
                    let column_type = if column.supports_copy(conn)? {
                        column.rust_data_type(conn)?
                    } else {
                        column.rust_ref_data_type(conn)?
                    };
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
            } else if !is_unique && arguments.len() == 1 && columns[0].supports_copy(conn)? && !columns[0].is_nullable() {
                let argument_types = columns
                    .iter()
                    .map(|column| {
                        let column = column.to_non_nullable();
                        column.rust_data_type(conn)
                    })
                    .collect::<Result<Vec<_>, WebCodeGenError>>()?;
                let formatted_argument_types = if argument_types.len() > 1 {
                    quote! { (#(#argument_types),*) }
                } else {
                    quote! { #(#argument_types)* }
                };
                let column_ident = columns[0].snake_case_ident()?;
                quote! {
                    pub fn #method_ident<C>(
                        #(#arguments,)*
                        conn: &mut C,
                    ) -> Result<#ok_return_type, diesel::result::Error>
                    where
                        C: diesel::connection::LoadConnection,
                        <Self as diesel::associations::HasTable>::Table: diesel::query_dsl::methods::FilterDsl<
                            <#table_diesel_path::#column_ident as diesel::expression_methods::EqAll<#formatted_argument_types>>::Output,
                        >,
                        <<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
                            <#table_diesel_path::#column_ident as diesel::expression_methods::EqAll<#formatted_argument_types>>::Output,
                        >>::Output: diesel::query_dsl::methods::OrderDsl<#formatted_primary_key_order_by_type>,
                        <<<Self as diesel::associations::HasTable>::Table as diesel::query_dsl::methods::FilterDsl<
                            <#table_diesel_path::#column_ident as diesel::expression_methods::EqAll<#formatted_argument_types>>::Output,
                        >>::Output as diesel::query_dsl::methods::OrderDsl<#formatted_primary_key_order_by_type>>::Output: diesel::RunQueryDsl<C> + for<'a> diesel::query_dsl::LoadQuery<'a, C, Self>,
                    {
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
