use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper, TextExpressionMethods,
};
use itertools::Itertools;
use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
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

    /// Writes the Syn TokenStream for the struct definition and associated methods to a file.
    pub fn write_syn_to_file(&self, conn: &mut PgConnection, output_path: &str) {
        let output = self.to_syn(conn);

        // Convert the generated TokenStream to a string
        let code_string = output.to_string();

        println!("{}", code_string);

        // Parse the generated code string into a syn::Item
        let syntax_tree: File = syn::parse_str(&code_string).unwrap();

        // Use prettyplease to format the syntax tree
        let formatted_code = unparse(&syntax_tree);

        // Write the formatted code to the output file
        std::fs::write(output_path, formatted_code).unwrap();
    }

    /// Returns the Syn TokenStream for the struct definition and associated methods.
    pub fn to_syn(&self, conn: &mut PgConnection) -> TokenStream {
        let struct_name: Ident = Ident::new(&self.struct_name(), proc_macro2::Span::call_site());
        let attributes = self.columns(conn).unwrap().into_iter().map(|column| {
            let column_attribute: Ident =
                Ident::new(&column.column_name, proc_macro2::Span::call_site());
            let column_type = column.rust_type();
            quote! {
                pub #column_attribute: #column_type,
            }
        });

        let foreign_key_methods = self
            .columns(conn)
            .unwrap()
            .into_iter()
            .filter_map(|column| {
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
                    pub fn #method_name(&self, conn: &mut PgConnection) -> Result<#foreign_key_struct_name, DieselError> {
                        use crate::schema::#foreign_key_table_name;
                        #foreign_key_table_name::dsl::#foreign_key_table_name
                            .filter(#foreign_key_table_name::dsl::#foreign_key_column_name.eq(self.#current_column_name))
                            .first::<#foreign_key_struct_name>(conn)
                    }
                })
            });

        quote! {
            #[derive(Debug)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            #[cfg_attr(feature = "diesel", derive(diesel::Queryable, diesel::Selectable))]
            pub struct #struct_name {
                #(#attributes)*
            }

            impl #struct_name {
                #(#foreign_key_methods)*
            }
        }
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

    pub fn load_all_tables(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::tables;
        tables::dsl::tables
            .filter(tables::dsl::table_catalog.eq(table_catalog))
            .filter(tables::dsl::table_schema.eq(table_schema.unwrap_or("public")))
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
