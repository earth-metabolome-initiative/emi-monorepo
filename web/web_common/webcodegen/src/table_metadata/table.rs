use std::collections::HashMap;

use diesel::{
    pg::PgConnection, result::Error as DieselError, BoolExpressionMethods, ExpressionMethods,
    JoinOnDsl, NullableExpressionMethods, QueryDsl, Queryable, QueryableByName, RunQueryDsl,
    Selectable, SelectableHelper,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;
use syn::{parse_str, Ident, Type};

use super::PgTrigger;
use crate::{errors::WebCodeGenError, CheckConstraint, Column, PgIndex, TableConstraint};

/// Reserved Rust words that cannot be used as identifiers.
pub const RESERVED_RUST_WORDS: [&str; 49] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", // Future reserved keywords
    "abstract", "async", "await", "become", "box", "do", "final", "macro", "override", "priv",
    "try", "typeof", "unsized", "virtual", "yield",
];

/// Diesel collisions that need to be handled.
pub const RESERVED_DIESEL_WORDS: [&str; 1] = ["columns"];

#[derive(Queryable, QueryableByName, PartialEq, Eq, Selectable, Debug, Clone, Hash)]
#[diesel(table_name = crate::schema::tables)]
/// Struct defining the `information_schema.tables` table.
pub struct Table {
    /// The catalog name of the table.
    pub table_catalog: String,
    /// The schema name of the table.
    pub table_schema: String,
    /// The name of the table.
    pub table_name: String,
    /// The type of the table, e.g. `BASE TABLE` or `VIEW`.
    pub table_type: String,
    /// The name of the table that the current table is a temporary table for.
    pub self_referencing_column_name: Option<String>,
    /// The name of the column that is a foreign key to the current table.
    pub reference_generation: Option<String>,
    /// The user-defined type catalog.
    pub user_defined_type_catalog: Option<String>,
    /// The user-defined type schema.
    pub user_defined_type_schema: Option<String>,
    /// The user-defined type name.
    pub user_defined_type_name: Option<String>,
    /// The user-defined type name that the current table is a temporary table
    /// for.
    pub is_insertable_into: String,
    /// Whether the table is typed.
    pub is_typed: String,
    /// Whether the table is updatable.
    pub commit_action: Option<String>,
}

impl Table {
    /// Returns the correct diesel feature flag for the number of columns in the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A string representing the diesel feature flag for the number of columns
    /// in the table.
    ///
    /// # Errors
    ///
    /// * If the number of columns exceeds 128.
    pub fn diesel_feature_flag_name(
        &self,
        conn: &mut PgConnection,
    ) -> Result<&str, WebCodeGenError> {
        let number_of_columns = self.columns(conn)?.len();
        if number_of_columns > 128 {
            Err(WebCodeGenError::ExcessiveNumberOfColumns(
                Box::new(self.clone()),
                number_of_columns,
            ))
        } else if number_of_columns > 64 {
            Ok("128-column-tables")
        } else if number_of_columns > 32 {
            Ok("64-column-tables")
        } else if number_of_columns > 16 {
            Ok("32-column-tables")
        } else {
            Ok("diesel")
        }
    }

    /// Returns the correct diesel feature flag for the number of columns in the
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the diesel feature flag for the number of
    /// columns in the table.
    ///
    /// # Errors
    ///
    /// * If the diesel feature flag name cannot be generated.
    pub fn diesel_column_feature_flag(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(quote! {#[cfg(feature = #flag_name)]})
    }

    #[must_use]
    /// Returns whether the table is a view.
    pub fn is_view(&self) -> bool {
        self.table_type == "VIEW"
    }

    #[must_use]
    /// Returns whether the table is temporary.
    pub fn is_temporary(&self) -> bool {
        self.table_type == "LOCAL TEMPORARY" || self.table_type == "GLOBAL TEMPORARY"
    }

    /// Returns the name of the struct converted from the table name.
    ///
    /// # Errors
    ///
    /// * If the camel case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the name of the struct converted from the table
    /// name.
    pub fn struct_name(&self) -> Result<String, WebCodeGenError> {
        let sanitizer = SnakeCaseSanizer::default()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_camel_case(&self.singular_table_name())?)
    }

    /// Returns the Rust Ident of the struct converted from the table name.
    ///
    /// # Errors
    ///
    /// * If the camel case name cannot be generated.
    pub fn struct_ident(&self) -> Result<Ident, WebCodeGenError> {
        let struct_name = self.struct_name()?;
        if RESERVED_RUST_WORDS.contains(&struct_name.as_str()) {
            Ok(Ident::new_raw(&struct_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&struct_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A string representing the sanitized snake case name of the table.
    pub fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        let sanitizer = SnakeCaseSanizer::default()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_snake_case(&self.table_name)?)
    }

    /// Returns whether the table has a sanitized snake case name.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has a sanitized snake case name.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    pub fn has_snake_case_name(&self) -> Result<bool, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        Ok(snake_case_name != self.table_name)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    ///
    /// # Returns
    ///
    /// A `Ident` representing the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    pub fn snake_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    #[must_use]
    /// Returns the singular name of the table.
    pub fn singular_table_name(&self) -> String {
        // We split the table name by underscores and remove the last element.
        let mut parts =
            self.table_name.split('_').map(|part| part.to_string()).collect::<Vec<String>>();
        let last_element = parts.pop().unwrap();
        // We convert to singular form the last element and join the parts back
        // together.
        let singular_last_element = pluralizer::pluralize(last_element.as_str(), 1, false);
        parts.push(singular_last_element);
        parts.join("_")
    }

    /// Returns wether a table require authorizations to be viewed
    pub fn require_view_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns wether a table require authorizations to be modified
    pub fn require_modify_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns the primary key identifiers for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of `Ident` representing the primary key identifiers.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_identifiers(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Ident>, WebCodeGenError> {
        self.primary_key_columns(conn)?
            .into_iter()
            .map(|column| column.sanitized_snake_case_ident())
            .collect::<Result<Vec<Ident>, WebCodeGenError>>()
    }

    /// Returns the primary key decorator to be used for this table, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the primary key decorator.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_decorator(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // In some cases, the table will not have a primary key. In which case, we
        // cannot specify the primary key decorator on the struct.
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(if self.has_primary_keys(conn)? {
            let primary_key_identifiers = self.primary_key_identifiers(conn)?;
            quote! {
                #[cfg_attr(feature = #columns_feature_flag_name, diesel(primary_key(#(#primary_key_identifiers),*)))]
            }
        } else {
            TokenStream::new()
        })
    }

    /// Returns the diesel derives supported by the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of `Type` representing the diesel derives.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn diesel_derives(&self, conn: &mut PgConnection) -> Result<Vec<Type>, WebCodeGenError> {
        let mut derives = Vec::new();

        derives.push(parse_str("diesel::Selectable").unwrap());

        if self.has_primary_keys(conn)? {
            derives.push(parse_str("diesel::Queryable").unwrap());
            derives.push(parse_str("diesel::Identifiable").unwrap());
        }

        Ok(derives)
    }

    /// Returns the diesel derives decorator for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A `TokenStream` representing the diesel derives decorator.
    ///
    /// # Errors
    ///
    /// * If the diesel derives cannot be loaded.
    pub fn diesel_derives_decorator(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let diesel_derives = self.diesel_derives(conn)?;
        let columns_feature_flag_name = self.diesel_feature_flag_name(conn)?;
        Ok(if diesel_derives.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                #[cfg_attr(feature = #columns_feature_flag_name, derive(#(#diesel_derives),*))]
            }
        })
    }

    /// Returns the foreign keys of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns representing the foreign keys of the table.
    pub fn foreign_keys(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        Ok(self
            .columns(conn)?
            .into_iter()
            .filter(|column| column.is_foreign_key(conn))
            .collect::<Vec<Column>>())
    }

    /// Returns whether the table has user-associated columns.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has user-associated columns.
    ///
    /// # Errors
    ///
    /// * If the columns cannot be loaded from the database.
    pub fn has_session_user_generated_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().any(|column| column.is_session_user_generated(conn)))
    }

    /// Returns whether the table IS the `users` table.
    pub fn is_users_table(&self) -> bool {
        self.table_name == "users"
    }

    /// Returns whether the table has an `created_by` column.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_created_by_column(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().any(|column| column.is_created_by(conn)))
    }

    /// Returns whether the table has an `updated_by` column.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_updated_by_column(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().any(|column| column.is_updated_by(conn)))
    }

    /// Returns the UNIQUE constraint indices for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of indices.
    ///
    /// # Errors
    ///
    /// * If the indices cannot be loaded from the database.
    pub fn unique_indices(&self, conn: &mut PgConnection) -> Result<Vec<PgIndex>, DieselError> {
        use crate::schema::{pg_class, pg_index};

        let (pg_class1, pg_class2) = diesel::alias!(pg_class as pg_class1, pg_class as pg_class2);

        pg_index::table
            .inner_join(pg_class1.on(pg_class1.field(pg_class::oid).eq(pg_index::indexrelid)))
            .inner_join(pg_class2.on(pg_class2.field(pg_class::oid).eq(pg_index::indrelid)))
            .filter(pg_class2.field(pg_class::relname).eq(&self.table_name).and(
                pg_class2.field(pg_class::relnamespace).eq(pg_class1.field(pg_class::relnamespace)),
            ))
            .filter(pg_index::indisunique.eq(true))
            .select(PgIndex::as_select())
            .load::<PgIndex>(conn)
    }

    /// Returns all tables in the database.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_catalog` - The table catalog.
    /// * `table_schema` - The table schema.
    ///
    /// # Returns
    ///
    /// A vector of all tables in the database.
    ///
    /// # Errors
    ///
    /// * If the tables cannot be loaded from the database.
    pub fn load_all(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, DieselError> {
        use crate::schema::tables;
        tables::table
            .filter(tables::table_catalog.eq(table_catalog))
            .filter(tables::table_schema.eq(table_schema.unwrap_or("public")))
            .filter(tables::table_name.ne("__diesel_schema_migrations"))
            .load::<Table>(conn)
    }

    /// Returns all tables in the database sorted topologically.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_catalog` - The table catalog.
    /// * `table_schema` - The table schema.
    ///
    /// # Returns
    ///
    /// A vector of all tables in the database sorted topologically, i.e.
    /// such that tables that depend on other tables are listed after the
    /// tables they depend on.
    pub fn load_all_topologically(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        let mut tables = Self::load_all(conn, table_catalog, table_schema)?;

        let mut table_priority: HashMap<Table, usize> =
            tables.iter().cloned().map(|table| (table, 0)).collect();

        // We determine the priority of each table by setting the priority of a table
        // to the maximum priority of the tables it depends on plus one.
        loop {
            let mut changed = false;
            for table in tables.iter() {
                for column in table.columns(conn)? {
                    if let Some((foreign_table, _)) = column.foreign_table(conn)? {
                        if foreign_table == *table {
                            continue;
                        }
                        let priority = table_priority.get(&foreign_table).unwrap() + 1;
                        table_priority.entry(table.clone()).and_modify(|e| {
                            if *e < priority {
                                *e = priority;
                                changed = true;
                            }
                        });
                    }
                }
            }
            if !changed {
                break;
            }
        }

        // We sort the tables by their priority.
        tables.sort_by(|a, b| table_priority.get(a).unwrap().cmp(table_priority.get(b).unwrap()));

        Ok(tables)
    }

    /// Returns the table by name.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `table_name` - The name of the table.
    /// * `table_schema` - The schema of the table.
    /// * `table_catalog` - The catalog of the table.
    pub fn load(
        conn: &mut PgConnection,
        table_name: &str,
        table_schema: Option<&str>,
        table_catalog: &str,
    ) -> Result<Self, DieselError> {
        use crate::schema::tables;
        let table_schema = table_schema.unwrap_or("public");
        tables::table
            .filter(tables::table_name.eq(table_name))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_catalog.eq(table_catalog))
            .first::<Table>(conn)
    }

    /// Returns the columns of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the columns cannot be loaded from the database.
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::columns;
        Ok(columns::table
            .filter(columns::table_name.eq(&self.table_name))
            .filter(columns::table_schema.eq(&self.table_schema))
            .filter(columns::table_catalog.eq(&self.table_catalog))
            .load::<Column>(conn)?)
    }

    /// Returns the column by name.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    /// * `column_name` - The name of the column.
    ///
    /// # Returns
    ///
    /// The column.
    ///
    /// # Errors
    ///
    /// * If the column cannot be loaded from the database.
    pub fn column_by_name(
        &self,
        conn: &mut PgConnection,
        column_name: &str,
    ) -> Result<Column, DieselError> {
        use crate::schema::columns;
        columns::table
            .filter(columns::table_name.eq(&self.table_name))
            .filter(columns::table_schema.eq(&self.table_schema))
            .filter(columns::table_catalog.eq(&self.table_catalog))
            .filter(columns::column_name.eq(column_name))
            .first::<Column>(conn)
    }

    /// Returns the groups of columns defining unique constraints.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of vectors of columns.
    ///
    /// # Errors
    ///
    /// * If the provided database connection is invalid.
    pub fn unique_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Vec<Column>>, WebCodeGenError> {
        use crate::schema::{columns, key_column_usage, table_constraints};
        Ok(key_column_usage::table
            .inner_join(
                columns::table.on(key_column_usage::table_name
                    .nullable()
                    .eq(columns::table_name.nullable())
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(columns::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(columns::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::column_name
                            .nullable()
                            .eq(columns::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::table.on(key_column_usage::constraint_name
                    .nullable()
                    .eq(table_constraints::constraint_name.nullable())
                    .and(
                        key_column_usage::constraint_schema
                            .nullable()
                            .eq(table_constraints::constraint_schema.nullable()),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .nullable()
                            .eq(table_constraints::constraint_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::table_name
                            .nullable()
                            .eq(table_constraints::table_name.nullable()),
                    )
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(table_constraints::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(table_constraints::table_catalog.nullable()),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::constraint_type.eq("UNIQUE"))
            .order_by(table_constraints::constraint_name)
            .select((TableConstraint::as_select(), Column::as_select()))
            .load::<(TableConstraint, Column)>(conn)
            .map(|rows| {
                rows.into_iter()
                    .chunk_by(|(constraint, _)| constraint.constraint_name.clone())
                    .into_iter()
                    .map(|(_, group)| {
                        group.into_iter().map(|(_, column)| column).collect::<Vec<Column>>()
                    })
                    .collect()
            })?)
    }

    /// Returns whether the table has primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has primary keys.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn has_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.primary_key_columns(conn).map(|columns| !columns.is_empty())
    }

    /// Returns the columns composing the primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of columns.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        use crate::schema::{columns, key_column_usage, table_constraints};
        Ok(key_column_usage::table
            .inner_join(
                columns::table.on(key_column_usage::table_name
                    .nullable()
                    .eq(columns::table_name.nullable())
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(columns::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(columns::table_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::column_name
                            .nullable()
                            .eq(columns::column_name.nullable()),
                    )),
            )
            .inner_join(
                table_constraints::table.on(key_column_usage::constraint_name
                    .nullable()
                    .eq(table_constraints::constraint_name.nullable())
                    .and(
                        key_column_usage::constraint_schema
                            .nullable()
                            .eq(table_constraints::constraint_schema.nullable()),
                    )
                    .and(
                        key_column_usage::constraint_catalog
                            .nullable()
                            .eq(table_constraints::constraint_catalog.nullable()),
                    )
                    .and(
                        key_column_usage::table_name
                            .nullable()
                            .eq(table_constraints::table_name.nullable()),
                    )
                    .and(
                        key_column_usage::table_schema
                            .nullable()
                            .eq(table_constraints::table_schema.nullable()),
                    )
                    .and(
                        key_column_usage::table_catalog
                            .nullable()
                            .eq(table_constraints::table_catalog.nullable()),
                    )),
            )
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .filter(table_constraints::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)?)
    }

    /// Returns the check constraints for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of check constraints.
    ///
    /// # Errors
    ///
    /// * If the check constraints cannot be loaded from the database.
    pub fn check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<CheckConstraint>, DieselError> {
        use crate::schema::{check_constraints, table_constraints};

        check_constraints::table
            .inner_join(
                table_constraints::table.on(check_constraints::constraint_name
                    .eq(table_constraints::constraint_name)
                    .and(
                        check_constraints::constraint_schema
                            .eq(table_constraints::constraint_schema),
                    )
                    .and(
                        check_constraints::constraint_catalog
                            .eq(table_constraints::constraint_catalog),
                    )),
            )
            .filter(table_constraints::table_name.eq(&self.table_name))
            .filter(table_constraints::table_schema.eq(&self.table_schema))
            .filter(table_constraints::table_catalog.eq(&self.table_catalog))
            .select(CheckConstraint::as_select())
            .load::<CheckConstraint>(conn)
    }

    /// Returns the list of Triggers associates to the current table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of triggers.
    ///
    /// # Errors
    ///
    /// * If the triggers cannot be loaded from the database.
    pub fn triggers(&self, conn: &mut PgConnection) -> Result<Vec<PgTrigger>, DieselError> {
        use crate::schema::{pg_class, pg_namespace, pg_trigger};
        pg_trigger::table
            .inner_join(pg_class::table.on(pg_trigger::tgrelid.eq(pg_class::oid)))
            .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
            .filter(pg_class::relname.eq(&self.table_name))
            .filter(pg_namespace::nspname.eq(&self.table_schema))
            .select(PgTrigger::as_select())
            .load::<PgTrigger>(conn)
    }
}
