use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods,
};
use itertools::Itertools;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{File, Ident};

use crate::Column;
use crate::Index;
use crate::TableConstraint;

/// Struct defining the `information_schema.tables` table.
#[derive(Queryable, QueryableByName, PartialEq, Eq, Selectable, Debug)]
#[diesel(table_name = crate::schema::tables)]
pub struct Table {
    pub table_catalog: String,
    pub table_schema: String,
    pub table_name: String,
    pub table_type: String,
    pub self_referencing_column_name: Option<String>,
    pub reference_generation: Option<String>,
    pub user_defined_type_catalog: Option<String>,
    pub user_defined_type_schema: Option<String>,
    pub user_defined_type_name: Option<String>,
    pub is_insertable_into: String,
    pub is_typed: String,
    pub commit_action: Option<String>,
}

impl Table {
    /// Returns the name of the struct converted from the table name.
    pub fn struct_name(&self) -> String {
        self.singular_table_name()
            .split('_')
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().chain(chars).collect(),
                }
            })
            .collect()
    }

    /// Returns the singular name of the table.
    pub fn singular_table_name(&self) -> String {
        // TODO: make singular
        self.table_name.clone()
    }

    /// Shared traits for all tables.
    pub fn shared_traits() -> TokenStream {
        quote! {
            pub trait Connection {
                type Error: std::error::Error;
            }

            #[cfg(feature = "diesel")]
            impl Connection for diesel_async::AsyncPgConnection {
                type Error = diesel::result::Error;
            }
        }
    }

    /// Defines the insertion-related traits.
    pub fn insert_variant_trait() -> TokenStream {
        quote! {
            #[cfg(feature = "diesel")]
            pub trait IntoSessionInsertVariant {
                type SessionInsertVariant: InsertableVariant<diesel_async::AsyncPgConnection>;

                fn into_session_insert_variant(self, created_by: i32) -> Self::SessionInsertVariant;
            }

            pub trait InsertableVariant<C: Connection> {
                type Row;

                async fn insert(&self, conn: &mut C) -> Result<Self::Row, C::Error>;
            }
        }
    }

    /// Defines the update-related traits.
    pub fn update_variant_trait() -> TokenStream {
        quote! {
            #[cfg(feature = "diesel")]
            pub trait IntoSessionUpdateVariant {
                type SessionUpdateVariant: UpdateableVariant<diesel_async::AsyncPgConnection>;

                fn into_session_update_variant(self, updated_by: i32) -> Self::SessionUpdateVariant;
            }

            pub trait UpdateableVariant<C: Connection> {
                type Row;

                async fn update(&self, conn: &mut C) -> Result<Self::Row, C::Error>;
            }
        }
    }

    /// Defines the HasFilterVariant trait.
    pub fn filter_variant_trait() -> TokenStream {
        quote! {
            pub trait HasFilterVariant {
                type FilterVariant;
            }
        }
    }

    /// Returns all relevant traits for the table.
    pub fn traits() -> TokenStream {
        let shared_traits = Self::shared_traits();
        let insert_variant_trait = Self::insert_variant_trait();
        let update_variant_trait = Self::update_variant_trait();
        let filter_variant_trait = Self::filter_variant_trait();
        quote! {
            #shared_traits
            #insert_variant_trait
            #update_variant_trait
            #filter_variant_trait
        }
    }

    /// Writes all the tables syn version to a file.
    pub fn write_all(
        conn: &mut PgConnection,
        output_path: &str,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<(), DieselError> {
        let tables = Table::load_all(conn, table_catalog, table_schema)?;

        let schema = tables
            .iter()
            .map(|table| table.to_schema(conn))
            .collect::<Result<Vec<TokenStream>, DieselError>>()?;

        let table_structs = tables
            .iter()
            .map(|table| table.to_syn(conn))
            .collect::<Result<Vec<TokenStream>, DieselError>>()?;

        let table_idents = tables
            .iter()
            .map(|table| Ident::new(&table.table_name, proc_macro2::Span::call_site()));

        let traits = Table::traits();

        // Create a new TokenStream
        let output = quote! {
            #[cfg(feature = "diesel")]
            use diesel::{ExpressionMethods, QueryDsl, SelectableHelper};
            #[cfg(feature = "diesel")]
            use diesel_async::RunQueryDsl;

            #( #schema )*

            #[cfg(feature = "diesel")]
            diesel::allow_tables_to_appear_in_same_query!( #( #table_idents ),* );

            #traits

            #( #table_structs )*
        };

        // Convert the generated TokenStream to a string
        let code_string = output.to_string();

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string).unwrap();

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        // Write the formatted code to the output file
        std::fs::write(output_path, formatted_code).unwrap();

        Ok(())
    }

    /// Returns the Syn TokenStream for the diesel schema definition for the table.
    pub fn to_schema(&self, conn: &mut PgConnection) -> Result<TokenStream, DieselError> {
        let table_name = Ident::new(&self.table_name, proc_macro2::Span::call_site());
        let table_schema = Ident::new(&self.table_schema, proc_macro2::Span::call_site());
        let columns = self.columns(conn)?.into_iter().map(|column| {
            let column_name = Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.diesel_type();
            quote! {
                #column_name -> #column_type
            }
        });
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| Ident::new(&column.column_name, proc_macro2::Span::call_site()));

        Ok(quote! {
            #[cfg(feature = "diesel")]
            diesel::table! {
                #table_schema.#table_name (#(#primary_key_names, )*) {
                    #(#columns),*
                }
            }
        })
    }

    /// Returns wether a table require authorizations to be viewed
    pub fn require_view_authorizations(&self, conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns wether a table require authorizations to be modified
    pub fn require_modify_authorizations(&self, conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns the Syn TokenStream for the struct definition and associated methods.
    pub fn to_syn(&self, conn: &mut PgConnection) -> Result<TokenStream, DieselError> {
        let table_name = Ident::new(&self.table_name, proc_macro2::Span::call_site());
        let struct_name: Ident = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let primary_key_columns = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| Ident::new(&column.column_name, proc_macro2::Span::call_site()));
        let attributes = self.columns(conn)?.into_iter().map(|column| {
            let column_attribute: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();
            quote! {
                pub #column_attribute: #column_type
            }
        });

        let mutability_impls = if self.has_session_user_generated_columns(conn) {
            let insert_trait_impls = self.insert_trait_impls(conn)?;
            let update_trait_impls = self.update_trait_impls(conn)?;
            quote! {
                #insert_trait_impls
                #update_trait_impls
            }
        } else {
            TokenStream::new()
        };
        let foreign_key_methods = self.foreign_key_methods(conn);

        Ok(quote! {
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Selectable, diesel::QueryableByName))]
            #[cfg_attr(feature = "diesel", diesel(table_name = #table_name))]
            #[cfg_attr(feature = "diesel", diesel(primary_key(#(#primary_key_columns),*)))]
            pub struct #struct_name {
                #(#attributes),*
            }

            impl #struct_name {
                #(#foreign_key_methods)*
            }

            #mutability_impls
        })
    }

    fn foreign_key_methods<'a>(
        &'a self,
        conn: &'a mut PgConnection,
    ) -> impl Iterator<Item = TokenStream> + 'a {
        self
            .columns(conn)
            .unwrap()
            .into_iter()
            .filter_map(move |column| {
                if !column.is_foreign_key(conn) {
                    return None;
                }
                let (foreign_key_table, foreign_key_column) = column.foreign_table(conn).unwrap().unwrap();
                let foreign_key_table_name: Ident = Ident::new(&foreign_key_table.table_name, proc_macro2::Span::call_site());
                let foreign_key_column_name: Ident = Ident::new(&foreign_key_column.column_name, proc_macro2::Span::call_site());
                let method_name: Ident = if column.column_name.ends_with("_id") {
                    Ident::new(&column.column_name[..column.column_name.len() - 3], proc_macro2::Span::call_site())
                } else {
                    Ident::new(&column.column_name, proc_macro2::Span::call_site())
                };
                let current_column_name: Ident = Ident::new(&column.column_name, proc_macro2::Span::call_site());
                let foreign_key_struct_name: Ident = Ident::new(&foreign_key_table.struct_name(), proc_macro2::Span::call_site());

                Some(quote! {
                    #[cfg(feature = "diesel")]
                    pub async fn #method_name(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<#foreign_key_struct_name, diesel::result::Error> {
                        #foreign_key_table_name::dsl::#foreign_key_table_name
                            .filter(#foreign_key_table_name::dsl::#foreign_key_column_name.eq(self.#current_column_name))
                            .select(#foreign_key_struct_name::as_select())
                            .first::<#foreign_key_struct_name>(conn)
                            .await
                    }
                })
            })
    }

    pub fn delete_method(&self, conn: &mut PgConnection) -> TokenStream {
        let struct_name: Ident = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let primary_key_columns = self.primary_key_columns(conn).unwrap();

        let where_clause = primary_key_columns
            .iter()
            .map(|column| {
                let column_name: Ident =
                    Ident::new(&column.column_name, proc_macro2::Span::call_site());
                quote! {
                    #struct_name::dsl::#column_name.eq(&self.#column_name)
                }
            })
            .collect::<Vec<_>>();

        // Join the where clauses with an and
        let where_clause = where_clause
            .into_iter()
            .reduce(|a, b| quote! { #a.and(#b) })
            .unwrap();

        quote! {
            #[cfg(feature = "diesel")]
            async pub fn delete(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<usize, DieselError> {
                diesel::delete(#struct_name.filter(#(#where_clause).and_then(|x| Ok(x)))).execute(conn).await
            }
        }
    }

    pub fn has_session_user_generated_columns(&self, conn: &mut PgConnection) -> bool {
        self.columns(conn)
            .unwrap()
            .iter()
            .any(|column| column.is_session_user_generated(conn))
    }

    pub fn insert_trait_impls(&self, conn: &mut PgConnection) -> Result<TokenStream, DieselError> {
        let struct_name: Ident = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let table_name = Ident::new(&self.table_name, proc_macro2::Span::call_site());
        let columns = self.columns(conn)?;

        let session_insert_columns = columns
            .iter()
            .filter(|column| !column.is_automatically_generated())
            .collect::<Vec<&Column>>();

        let insert_columns = session_insert_columns
            .iter()
            .filter(|column| !column.is_session_user_generated(conn))
            .collect::<Vec<&&Column>>();

        let session_insert_variant_name = Ident::new(
            &format!("SessionInsert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let session_insert_variable_attributes = session_insert_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();
            quote! {
                pub #column_name: #column_type,
            }
        });

        let insert_variant_name = Ident::new(
            &format!("Insert{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let insertable_variant_attributes = insert_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();
            quote! {
                pub #column_name: #column_type,
            }
        });

        let into_session_insert_variant_map = insert_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        let new_structs_implementation = quote! {
            #[cfg(feature = "diesel")]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "diesel", derive(diesel::Insertable))]
            #[cfg_attr(feature = "diesel", diesel(table_name = #table_name))]
            pub struct #session_insert_variant_name {
                #(#session_insert_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #insert_variant_name {
                #(#insertable_variant_attributes)*
            }
        };

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = "diesel")]
            impl IntoSessionInsertVariant for #insert_variant_name {
                type SessionInsertVariant = #session_insert_variant_name;

                fn into_session_insert_variant(self, created_by: i32) -> Self::SessionInsertVariant {
                    #session_insert_variant_name {
                        #(#into_session_insert_variant_map)*
                        created_by,
                        updated_by: created_by
                    }
                }
            }

            #[cfg(feature = "diesel")]
            impl InsertableVariant<diesel_async::AsyncPgConnection> for #session_insert_variant_name {
                type Row = #struct_name;

                async fn insert(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<Self::Row, diesel::result::Error> {
                    diesel::insert_into(#table_name::dsl::#table_name)
                        .values(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }

    pub fn update_trait_impls(&self, conn: &mut PgConnection) -> Result<TokenStream, DieselError> {
        let struct_name: Ident = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let table_name = Ident::new(&self.table_name, proc_macro2::Span::call_site());
        let columns = self.columns(conn)?;
        let primary_key_names = self
            .primary_key_columns(conn)?
            .into_iter()
            .map(|column| column.column_name.clone())
            .collect::<Vec<String>>();

        let primary_key_identifiers = primary_key_names
            .iter()
            .map(|column_name| Ident::new(column_name, proc_macro2::Span::call_site()));

        let session_update_columns = columns
            .iter()
            .filter(|column| {
                (!column.is_automatically_generated()
                    || primary_key_names.contains(&column.column_name))
                    && !column.is_created_by(conn)
            })
            .collect::<Vec<&Column>>();

        let update_columns = session_update_columns
            .iter()
            .filter(|column| {
                !column.is_session_user_generated(conn)
                    || primary_key_names.contains(&column.column_name)
            })
            .collect::<Vec<&&Column>>();

        let session_update_variant_name = Ident::new(
            &format!("SessionUpdate{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let session_update_variable_attributes = session_update_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();
            if column.is_updated_by(conn) || primary_key_names.contains(&column.column_name) {
                quote! {
                    pub #column_name: #column_type,
                }
            } else {
                quote! {
                    pub #column_name: Option<#column_type>,
                }
            }
        });

        let update_variant_name = Ident::new(
            &format!("Update{struct_name}"),
            proc_macro2::Span::call_site(),
        );
        let updateable_variant_attributes = update_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();

            if primary_key_names.contains(&column.column_name) {
                quote! {
                    pub #column_name: #column_type,
                }
            } else {
                quote! {
                    pub #column_name: Option<#column_type>,
                }
            }
        });

        let into_session_update_variant_map = update_columns.iter().map(|column| {
            let column_name: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            quote! {
                #column_name: self.#column_name,
            }
        });

        let new_structs_implementation = quote! {
            #[cfg(feature = "diesel")]
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "diesel", derive(diesel::AsChangeset, diesel::Identifiable))]
            #[cfg_attr(feature = "diesel", diesel(primary_key(#(#primary_key_identifiers),*)))]
            #[cfg_attr(feature = "diesel", diesel(table_name = #table_name))]
            pub struct #session_update_variant_name {
                #(#session_update_variable_attributes)*
            }

            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #update_variant_name {
                #(#updateable_variant_attributes)*
            }
        };

        Ok(quote! {
            #new_structs_implementation

            #[cfg(feature = "diesel")]
            impl IntoSessionUpdateVariant for #update_variant_name {
                type SessionUpdateVariant = #session_update_variant_name;

                fn into_session_update_variant(self, updated_by: i32) -> Self::SessionUpdateVariant {
                    #session_update_variant_name {
                        #(#into_session_update_variant_map)*
                        updated_by
                    }
                }
            }

            #[cfg(feature = "diesel")]
            impl UpdateableVariant<diesel_async::AsyncPgConnection> for #session_update_variant_name {
                type Row = #struct_name;

                async fn update(&self, conn: &mut diesel_async::AsyncPgConnection) -> Result<Self::Row, diesel::result::Error> {
                    diesel::update(#table_name::dsl::#table_name)
                        .filter(#table_name::dsl::id.eq(self.id))
                        .set(self)
                        .get_result(conn)
                        .await
                }
            }
        })
    }

    pub fn unique_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
            .filter(pg_indexes::dsl::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::dsl::tablename.eq(&self.table_name))
            .filter(pg_indexes::dsl::indexdef.like("%UNIQUE%"))
            .load::<Index>(conn)
    }

    pub fn gin_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
            .filter(pg_indexes::dsl::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::dsl::tablename.eq(&self.table_name))
            .filter(pg_indexes::dsl::indexdef.like("%USING gin%"))
            .load::<Index>(conn)
    }

    pub fn gist_indexes(&self, conn: &mut PgConnection) -> Result<Vec<Index>, DieselError> {
        use crate::schema::pg_indexes;
        pg_indexes::dsl::pg_indexes
            .filter(pg_indexes::dsl::schemaname.eq(&self.table_schema))
            .filter(pg_indexes::dsl::tablename.eq(&self.table_name))
            .filter(pg_indexes::dsl::indexdef.like("%USING gist%"))
            .load::<Index>(conn)
    }

    pub fn load_all(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::tables;
        tables::dsl::tables
            .filter(tables::dsl::table_catalog.eq(table_catalog))
            .filter(tables::dsl::table_schema.eq(table_schema.unwrap_or("public")))
            .filter(tables::dsl::table_name.ne("__diesel_schema_migrations"))
            .load::<Table>(conn)
    }

    pub fn load(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Option<Self> {
        use crate::schema::tables;
        let table_schema = table_schema.unwrap_or("public");
        tables::dsl::tables
            .filter(tables::dsl::table_name.eq(table_name))
            .filter(tables::dsl::table_schema.eq(table_schema))
            .filter(tables::dsl::table_catalog.eq(table_catalog))
            .first::<Table>(conn)
            .ok()
    }

    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, DieselError> {
        use crate::schema::columns;
        columns::dsl::columns
            .filter(columns::dsl::table_name.eq(&self.table_name))
            .filter(columns::dsl::table_schema.eq(&self.table_schema))
            .filter(columns::dsl::table_catalog.eq(&self.table_catalog))
            .load::<Column>(conn)
    }

    pub fn column_by_name(
        &self,
        conn: &mut PgConnection,
        column_name: &str,
    ) -> Result<Column, DieselError> {
        use crate::schema::columns;
        columns::dsl::columns
            .filter(columns::dsl::table_name.eq(&self.table_name))
            .filter(columns::dsl::table_schema.eq(&self.table_schema))
            .filter(columns::dsl::table_catalog.eq(&self.table_catalog))
            .filter(columns::dsl::column_name.eq(column_name))
            .first::<Column>(conn)
    }

    pub fn unique_columns(&self, conn: &mut PgConnection) -> Result<Vec<Vec<Column>>, DieselError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        key_column_usage::dsl::key_column_usage
            .inner_join(
                columns::dsl::columns.on(key_column_usage::dsl::table_name
                    .nullable()
                    .eq(columns::dsl::table_name.nullable())
                    .and(
                        key_column_usage::dsl::table_schema
                            .nullable()
                            .eq(columns::dsl::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::table_catalog
                            .nullable()
                            .eq(columns::dsl::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::column_name
                            .nullable()
                            .eq(columns::dsl::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::dsl::table_constraints.on(
                    key_column_usage::dsl::constraint_name
                        .nullable()
                        .eq(table_constraints::dsl::constraint_name.nullable())
                        .and(
                            key_column_usage::dsl::constraint_schema
                                .nullable()
                                .eq(table_constraints::dsl::constraint_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::constraint_catalog
                                .nullable()
                                .eq(table_constraints::dsl::constraint_catalog.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_name
                                .nullable()
                                .eq(table_constraints::dsl::table_name.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_schema
                                .nullable()
                                .eq(table_constraints::dsl::table_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_catalog
                                .nullable()
                                .eq(table_constraints::dsl::table_catalog.nullable()),
                        ),
                ),
            )
            .filter(key_column_usage::dsl::table_name.eq(&self.table_name))
            .filter(key_column_usage::dsl::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::dsl::constraint_type.eq("UNIQUE"))
            .order_by(table_constraints::dsl::constraint_name)
            .select((TableConstraint::as_select(), Column::as_select()))
            .load::<(TableConstraint, Column)>(conn)
            .map(|rows| {
                rows.into_iter()
                    .chunk_by(|(constraint, _)| constraint.constraint_name.clone())
                    .into_iter()
                    .map(|(_, group)| {
                        group
                            .into_iter()
                            .map(|(_, column)| column)
                            .collect::<Vec<Column>>()
                    })
                    .collect()
            })
    }

    pub fn primary_key_columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, DieselError> {
        use crate::schema::columns;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        key_column_usage::dsl::key_column_usage
            .inner_join(
                columns::dsl::columns.on(key_column_usage::dsl::table_name
                    .nullable()
                    .eq(columns::dsl::table_name.nullable())
                    .and(
                        key_column_usage::dsl::table_schema
                            .nullable()
                            .eq(columns::dsl::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::table_catalog
                            .nullable()
                            .eq(columns::dsl::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::dsl::column_name
                            .nullable()
                            .eq(columns::dsl::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::dsl::table_constraints.on(
                    key_column_usage::dsl::constraint_name
                        .nullable()
                        .eq(table_constraints::dsl::constraint_name.nullable())
                        .and(
                            key_column_usage::dsl::constraint_schema
                                .nullable()
                                .eq(table_constraints::dsl::constraint_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::constraint_catalog
                                .nullable()
                                .eq(table_constraints::dsl::constraint_catalog.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_name
                                .nullable()
                                .eq(table_constraints::dsl::table_name.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_schema
                                .nullable()
                                .eq(table_constraints::dsl::table_schema.nullable()),
                        )
                        .and(
                            key_column_usage::dsl::table_catalog
                                .nullable()
                                .eq(table_constraints::dsl::table_catalog.nullable()),
                        ),
                ),
            )
            .filter(key_column_usage::dsl::table_name.eq(&self.table_name))
            .filter(key_column_usage::dsl::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::dsl::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::dsl::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)
    }
}
