use std::collections::{HashMap, HashSet};

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper, pg::PgConnection,
    result::Error as DieselError,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, parse_str};

use super::PgTrigger;
use crate::{
    CheckConstraint, Column, PgIndex, TableConstraint,
    codegen::{
        CODEGEN_DIESEL_MODULE, CODEGEN_DIRECTORY, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
};

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

#[derive(
    Queryable, QueryableByName, PartialEq, Eq, PartialOrd, Ord, Selectable, Debug, Clone, Hash,
)]
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
        crate::utils::struct_name(&self.table_name)
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
        crate::utils::snake_case_name(&self.table_name)
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
        Ok(snake_case_name == self.table_name)
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
            .map(|column| column.snake_case_ident())
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
        Ok(if self.has_primary_keys(conn)? {
            let primary_key_identifiers = self.primary_key_identifiers(conn)?;
            quote! {
                #[diesel(primary_key(#(#primary_key_identifiers),*))]
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

        derives.push(parse_str("diesel::Selectable")?);
        derives.push(parse_str("diesel::Insertable")?);

        if self.has_non_primary_keys(conn)? {
            derives.push(parse_str("diesel::AsChangeset")?);
        }

        if self.has_primary_keys(conn)? {
            derives.push(parse_str("diesel::Queryable")?);
            derives.push(parse_str("diesel::Identifiable")?);
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
        Ok(if diesel_derives.is_empty() {
            TokenStream::new()
        } else {
            quote! {
                #[derive(#(#diesel_derives),*)]
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
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn foreign_keys(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        Ok(self
            .columns(conn)?
            .into_iter()
            .filter(|column| column.is_foreign_key(conn))
            .collect::<Vec<Column>>())
    }

    /// Returns the parent keys of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The set of foreign key columns that have `ON DELETE CASCADE` constraint.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn parent_keys(&self, conn: &mut PgConnection) -> Result<Vec<Column>, WebCodeGenError> {
        Ok(self
            .columns(conn)?
            .into_iter()
            .filter(|column| column.is_foreign_key_on_delete_cascade(conn))
            .collect::<Vec<Column>>())
    }

    /// Returns the parent tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A vector of tables that are parents to the current table.
    ///
    /// # Errors
    ///
    /// * If the parent tables cannot be loaded from the database.
    pub fn parent_tables(&self, conn: &mut PgConnection) -> Result<Vec<Table>, WebCodeGenError> {
        let mut tables = Vec::new();
        for column in self.parent_keys(conn)? {
            if let Some((foreign_table, _)) = column.foreign_table(conn)? {
                if !tables.contains(&foreign_table) {
                    tables.push(foreign_table);
                }
            }
        }
        Ok(tables)
    }

    /// Returns whether the table has parent tables.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has parent tables.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn has_parents(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(!self.parent_keys(conn)?.is_empty())
    }

    /// Returns the set of foreign tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A set of tables that are foreign to the current table.
    ///
    ///
    /// # Errors
    ///
    /// * If the foreign tables cannot be loaded from the database.
    pub fn foreign_tables(&self, conn: &mut PgConnection) -> Result<Vec<Table>, WebCodeGenError> {
        let mut tables = HashSet::new();
        for column in self.foreign_keys(conn)? {
            if let Some((foreign_table, _)) = column.foreign_table(conn)? {
                tables.insert(foreign_table);
            }
        }
        Ok(tables.into_iter().collect())
    }

    /// Returns the set of children tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A set of tables that are children to the current table.
    ///
    /// # Errors
    ///
    /// * If the children tables cannot be loaded from the database.
    pub fn children_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, diesel::result::Error> {
        use crate::schema::{key_column_usage, referential_constraints, table_constraints, tables};
        tables::table
            .inner_join(
                table_constraints::table.on(tables::table_catalog
                    .eq(table_constraints::table_catalog)
                    .and(tables::table_schema.eq(table_constraints::table_schema))
                    .and(tables::table_name.eq(table_constraints::table_name))),
            )
            .inner_join(
                referential_constraints::table.on(table_constraints::constraint_catalog
                    .eq(referential_constraints::constraint_catalog)
                    .and(
                        table_constraints::constraint_schema
                            .eq(referential_constraints::constraint_schema),
                    )
                    .and(
                        table_constraints::constraint_name
                            .eq(referential_constraints::constraint_name),
                    )),
            )
            .inner_join(
                key_column_usage::table.on(referential_constraints::unique_constraint_catalog
                    .eq(key_column_usage::constraint_catalog.nullable())
                    .and(
                        referential_constraints::unique_constraint_schema
                            .eq(key_column_usage::constraint_schema.nullable()),
                    )
                    .and(
                        referential_constraints::unique_constraint_name
                            .eq(key_column_usage::constraint_name.nullable()),
                    )),
            )
            .filter(
                key_column_usage::table_catalog
                    .eq(&self.table_catalog)
                    .and(key_column_usage::table_schema.eq(&self.table_schema))
                    .and(key_column_usage::table_name.eq(&self.table_name)),
            )
            .select(tables::all_columns)
            .distinct()
            .load(conn)
    }

    /// Returns the set of sibling tables of the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A set of tables that are siblings to the current table.
    ///
    /// # Errors
    ///
    /// * If the sibling tables cannot be loaded from the database.
    pub fn sibling_tables(&self, conn: &mut PgConnection) -> Result<Vec<Table>, WebCodeGenError> {
        let mut tables = HashSet::new();
        for parent_table in self.parent_tables(conn)? {
            for child_table in parent_table.children_tables(conn)? {
                if child_table != *self {
                    tables.insert(child_table);
                }
            }
        }
        Ok(tables.into_iter().collect())
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

    /// Returns whether the table supports the `Copy` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If database connection fails.
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().all(|column| column.supports_copy(conn).unwrap_or(false)))
    }

    /// Returns whether the table supports the `Eq` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If database connection fails.
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().all(|column| column.supports_eq(conn).unwrap_or(false)))
    }

    /// Returns whether the table supports the `Ord` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If database connection fails.
    pub fn supports_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().all(|column| column.supports_ord(conn).unwrap_or(false)))
    }

    /// Returns whether the table supports the `Hash` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If database connection fails.
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.columns(conn)?.iter().all(|column| column.supports_hash(conn).unwrap_or(false)))
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
    ///
    /// # Errors
    ///
    /// * If the tables cannot be loaded from the database.
    pub fn load_all_topologically(
        conn: &mut PgConnection,
        table_catalog: &str,
        table_schema: Option<&str>,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        let mut tables = Self::load_all(conn, table_catalog, table_schema)?;

        let mut table_priority: HashMap<Table, usize> = HashMap::new();

        // We determine the priority of each table by setting the priority of a table
        // to the maximum priority of the tables it depends on plus one.
        loop {
            let mut changed = false;
            for table in &tables {
                for column in table.columns(conn)? {
                    if let Some((foreign_table, _)) = column.foreign_table(conn)? {
                        if foreign_table == *table {
                            continue;
                        }
                        let priority =
                            table_priority.get(&foreign_table).copied().unwrap_or_default() + 1;
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
        tables.sort_by(|a, b| {
            table_priority
                .get(a)
                .copied()
                .unwrap_or_default()
                .cmp(&table_priority.get(b).copied().unwrap_or_default())
        });

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
    ///
    /// # Returns
    ///
    /// The table.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
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

    /// Returns whether the table has columns that are NOT primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the provided database connection is invalid.
    pub fn has_non_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.non_primary_key_columns(conn).map(|columns| !columns.is_empty())
    }

    /// Returns the primary key type for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The syn data type representing the primary key type.
    ///
    /// # Errors
    ///
    /// * If the table does not have primary keys.
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        let primary_key_columns = self.primary_key_columns(conn)?;

        if primary_key_columns.is_empty() {
            return Err(WebCodeGenError::NoPrimaryKeyColumn(Box::new(self.clone())));
        }

        // We construct the rust type or tuple of rust types that represent the primary
        // key.
        Ok(if primary_key_columns.len() == 1 {
            // If the primary key is a single column, we can just use the type of that
            // column.
            primary_key_columns[0].rust_data_type(conn)?
        } else {
            // If the primary key is a composite key, we need to construct a tuple of the
            // types.
            let primary_key_types = primary_key_columns
                .iter()
                .map(|column| column.rust_data_type(conn))
                .collect::<Result<Vec<Type>, WebCodeGenError>>()?;
            syn::parse_quote! { (#(#primary_key_types),*) }
        })
    }

    /// Returns the primary key attribute(s) for the table.
    ///
    /// # Arguments
    ///
    /// * `include_self` - Whether to include the table self.
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// The syn [`TokenStream`](proc_macro2::TokenStream) representing the
    /// primary key attribute(s).
    ///
    /// # Errors
    ///
    /// * If the table does not have primary keys.
    /// * If the primary key columns cannot be loaded from the database.
    pub fn primary_key_attributes(
        &self,
        include_self: bool,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let primary_key_columns = self.primary_key_columns(conn)?;

        // We construct the rust type or tuple of rust types that represent the primary
        // key.
        Ok(if primary_key_columns.len() == 1 {
            // If the primary key is a single column, we can just use the type of that
            // column.
            let primary_key = primary_key_columns[0].snake_case_ident()?;
            if include_self {
                quote! { self.#primary_key }
            } else {
                quote! { #primary_key }
            }
        } else {
            // If the primary key is a composite key, we need to construct a tuple of the
            // types.
            let primary_key_names = primary_key_columns
                .iter()
                .map(Column::snake_case_ident)
                .collect::<Result<Vec<Ident>, WebCodeGenError>>()?;
            if include_self {
                quote! { (#(self.#primary_key_names),*) }
            } else {
                quote! { (#(#primary_key_names),*) }
            }
        })
    }

    /// Returns the columns NOT composing the primary keys.
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
    /// * If the connection to the database fails.
    pub fn non_primary_key_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut columns = self.columns(conn)?;
        let primary_key_columns = self.primary_key_columns(conn)?;
        columns.retain(|column| !primary_key_columns.contains(column));
        Ok(columns)
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

    /// Returns all multi column check constraints associated to the current
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the database.
    pub fn multi_column_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<CheckConstraint>, WebCodeGenError> {
        let mut multi_column_check_constraints = vec![];
        for check_constraint in self.check_constraints(conn)? {
            if check_constraint.is_multi_column_constraint(conn)? {
                multi_column_check_constraints.push(check_constraint);
            }
        }
        Ok(multi_column_check_constraints)
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

    /// Returns a the path to the diesel table module.
    ///
    /// # Returns
    ///
    /// A `syn::Type` representing the path to the diesel table module.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    pub fn import_diesel_path(&self) -> Result<syn::Type, WebCodeGenError> {
        let table_name = self.snake_case_name()?;
        Ok(syn::parse_str::<Type>(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_DIESEL_MODULE}::{CODEGEN_TABLES_PATH}::{table_name}::{table_name}",
        ))?)
    }

    /// Returns a the path to the table struct.
    ///
    /// # Returns
    ///
    /// A `syn::Type` representing the path to the table struct.
    ///
    /// # Errors
    ///
    /// * If the snake case name cannot be generated.
    pub fn import_struct_path(&self) -> Result<syn::Type, WebCodeGenError> {
        let table_name = self.snake_case_name()?;
        Ok(syn::parse_str::<Type>(&format!(
            "crate::{CODEGEN_DIRECTORY}::{CODEGEN_STRUCTS_MODULE}::{CODEGEN_TABLES_PATH}::{}::{}",
            table_name,
            self.struct_name()?
        ))?)
    }
}
