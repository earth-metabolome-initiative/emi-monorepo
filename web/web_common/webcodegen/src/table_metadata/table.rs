use std::collections::{HashMap, HashSet};

use cached::proc_macro::cached;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, PgConnection,
    QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Type, parse_str};

use super::{KeyColumnUsage, PgTrigger};
use crate::{
    CheckConstraint, Column, PgIndex, TableConstraint,
    codegen::{
        CODEGEN_DIESEL_MODULE, CODEGEN_DIRECTORY, CODEGEN_STRUCTS_MODULE, CODEGEN_TABLES_PATH,
    },
    errors::WebCodeGenError,
};

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn columns(table: &Table, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::columns;
    columns::table
        .filter(columns::table_name.eq(&table.table_name))
        .filter(columns::table_schema.eq(&table.table_schema))
        .filter(columns::table_catalog.eq(&table.table_catalog))
        .order_by(columns::ordinal_position)
        .load::<Column>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn children_tables(
    table: &Table,
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
                    table_constraints::constraint_name.eq(referential_constraints::constraint_name),
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
                .eq(&table.table_catalog)
                .and(key_column_usage::table_schema.eq(&table.table_schema))
                .and(key_column_usage::table_name.eq(&table.table_name)),
        )
        .select(Table::as_select())
        .load(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn unique_columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<Vec<Column>>, diesel::result::Error> {
    use crate::schema::{columns, key_column_usage, table_constraints};
    key_column_usage::table
        .inner_join(
            columns::table.on(key_column_usage::table_name
                .nullable()
                .eq(columns::table_name.nullable())
                .and(key_column_usage::table_schema.nullable().eq(columns::table_schema.nullable()))
                .and(
                    key_column_usage::table_catalog
                        .nullable()
                        .eq(columns::table_catalog.nullable()),
                )
                .and(key_column_usage::column_name.nullable().eq(columns::column_name.nullable()))),
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
        .filter(key_column_usage::table_name.eq(&table.table_name))
        .filter(key_column_usage::table_schema.eq(&table.table_schema))
        .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
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
        })
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn primary_key_columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, diesel::result::Error> {
    use crate::schema::{columns, key_column_usage, table_constraints};
    key_column_usage::table
        .inner_join(
            columns::table.on(key_column_usage::table_name
                .nullable()
                .eq(columns::table_name.nullable())
                .and(key_column_usage::table_schema.nullable().eq(columns::table_schema.nullable()))
                .and(
                    key_column_usage::table_catalog
                        .nullable()
                        .eq(columns::table_catalog.nullable()),
                )
                .and(key_column_usage::column_name.nullable().eq(columns::column_name.nullable()))),
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
        .filter(key_column_usage::table_name.eq(&table.table_name))
        .filter(key_column_usage::table_schema.eq(&table.table_schema))
        .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
        .filter(table_constraints::constraint_type.eq("PRIMARY KEY"))
        .select(Column::as_select())
        .load::<Column>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn foreign_keys(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<KeyColumnUsage>, diesel::result::Error> {
    use crate::schema::{key_column_usage, referential_constraints};
    key_column_usage::table
        .inner_join(
            referential_constraints::table.on(key_column_usage::constraint_name
                .eq(referential_constraints::constraint_name)
                .and(
                    key_column_usage::constraint_schema
                        .eq(referential_constraints::constraint_schema),
                )
                .and(
                    key_column_usage::constraint_catalog
                        .eq(referential_constraints::constraint_catalog),
                )),
        )
        .filter(key_column_usage::table_name.eq(&table.table_name))
        .filter(key_column_usage::table_schema.eq(&table.table_schema))
        .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
        .filter(key_column_usage::ordinal_position.eq(1))
        .select(KeyColumnUsage::as_select())
        .load::<KeyColumnUsage>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn unique_indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, diesel::result::Error> {
    use crate::schema::{pg_class, pg_index};

    let (pg_class1, pg_class2) = diesel::alias!(pg_class as pg_class1, pg_class as pg_class2);

    pg_index::table
        .inner_join(pg_class1.on(pg_class1.field(pg_class::oid).eq(pg_index::indexrelid)))
        .inner_join(pg_class2.on(pg_class2.field(pg_class::oid).eq(pg_index::indrelid)))
        .filter(pg_class2.field(pg_class::relname).eq(&table.table_name).and(
            pg_class2.field(pg_class::relnamespace).eq(pg_class1.field(pg_class::relnamespace)),
        ))
        .filter(pg_index::indisunique.eq(true))
        .select(PgIndex::as_select())
        .load::<PgIndex>(conn)
}

#[cached(
    result = true,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table.table_catalog, table.table_schema, table.table_name) }"#
)]
fn same_as_indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, diesel::result::Error> {
    Ok(unique_indices(table, conn)?
        .into_iter()
        .filter(|pg_index| pg_index.is_same_as(conn).map(|fk| fk.is_some()).unwrap_or(false))
        .collect())
}

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
    /// Returns whether the provided column is from the current table.
    ///
    /// # Arguments
    ///
    /// * `column` - The column to check.
    pub fn has_column(&self, column: &Column) -> bool {
        self.table_name == column.table_name
            && self.table_schema == column.table_schema
            && self.table_catalog == column.table_catalog
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
    pub fn foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, diesel::result::Error> {
        foreign_keys(self, conn)
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
    pub fn parent_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut parent_keys = Vec::new();
        for foreign_key in self.foreign_keys(conn)? {
            if foreign_key.is_on_delete_cascade(conn)?
                && foreign_key.is_foreign_primary_key(conn)?
                && !foreign_key.is_self_referential(conn)?
            {
                parent_keys.push(foreign_key);
            }
        }
        Ok(parent_keys)
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
        for foreign_key in self.parent_keys(conn)? {
            if let Some(foreign_table) = foreign_key.foreign_table(conn)? {
                if !tables.contains(&foreign_table) {
                    tables.push(foreign_table);
                }
            }
        }
        Ok(tables)
    }

    /// Returns the foreign key columns which point to the current table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn homogeneous_parent_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Vec<Column>>, WebCodeGenError> {
        let mut homogeneous_parent_columns = Vec::new();
        for foreign_key in self.foreign_keys(conn)? {
            if let Some(foreign_table) = foreign_key.foreign_table(conn)? {
                if foreign_table == *self && foreign_key.is_foreign_primary_key(conn)? {
                    homogeneous_parent_columns.push(foreign_key.columns(conn)?);
                }
            }
        }

        Ok(homogeneous_parent_columns)
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
        for foreign_key_constraint in self.foreign_keys(conn)? {
            if let Some(foreign_table) = foreign_key_constraint.foreign_table(conn)? {
                tables.insert(foreign_table);
            }
        }
        let mut tables = tables.into_iter().collect::<Vec<Table>>();
        tables.sort_unstable();
        Ok(tables)
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
        children_tables(self, conn)
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

    /// Returns whether the table has an `created_by` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_created_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        for column in self.columns(conn)? {
            if column.is_created_by(conn)? {
                return Ok(true);
            }
        }

        if include_ancestors {
            for table in self.extension_tables(conn)? {
                if table.has_created_by_column(true, conn)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Returns whether the table has an `updated_by` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///   search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_updated_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        for column in self.columns(conn)? {
            if column.is_updated_by(conn)? {
                return Ok(true);
            }
        }

        if include_ancestors {
            for table in self.extension_tables(conn)? {
                if table.has_updated_by_column(true, conn)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
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
    pub fn unique_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, diesel::result::Error> {
        unique_indices(self, conn)
    }

    /// Returns the same as indices for the table.
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
    pub fn same_as_indices(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgIndex>, diesel::result::Error> {
        same_as_indices(self, conn)
    }

    /// Returns the same as foreign keys for the table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn same_as_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(KeyColumnUsage, PgIndex)>, WebCodeGenError> {
        let mut same_as_foreign_keys = Vec::new();
        for foreign_key in self.foreign_keys(conn)? {
            if let Some(index) = foreign_key.is_same_as_constraint(conn)? {
                same_as_foreign_keys.push((foreign_key, index));
            }
        }
        Ok(same_as_foreign_keys)
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
    ) -> Result<Vec<Self>, diesel::result::Error> {
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
        for column in self.columns(conn)? {
            if !column.supports_copy(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
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
        for column in self.columns(conn)? {
            if !column.supports_eq(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
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
        for column in self.columns(conn)? {
            if !column.supports_ord(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
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
        for column in self.columns(conn)? {
            if !column.supports_hash(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
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
                    for foreign_key in column.foreign_keys(conn)? {
                        if let Some(foreign_table) = foreign_key.foreign_table(conn)? {
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
    ) -> Result<Self, diesel::result::Error> {
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
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Vec<Column>, diesel::result::Error> {
        columns(self, conn)
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
    ) -> Result<Column, diesel::result::Error> {
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
    ) -> Result<Vec<Vec<Column>>, diesel::result::Error> {
        unique_columns(self, conn)
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
    pub fn has_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
        self.primary_key_columns(conn).map(|columns| !columns.is_empty())
    }

    /// Returns whether the table has any other table extending it.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the database connection is invalid.
    pub fn is_extended(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        for child_table in self.children_tables(conn)? {
            if child_table.extension_tables(conn)?.contains(self) {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Returns the KeyColumnUsage linking to extension tables, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn extension_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .into_iter()
            .filter(|foreign_key| foreign_key.is_extension(conn).unwrap_or(false))
            .collect())
    }

    /// Returns the immediate tables this table extends, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    ///
    /// # Implementation details
    ///
    /// A table is considered an extension table if it has a primary key
    /// composed of a single column which is also a foreign key to another
    /// table. This does apply to composite primary keys.
    pub fn extension_tables(&self, conn: &mut PgConnection) -> Result<Vec<Table>, WebCodeGenError> {
        Ok(self
            .extension_foreign_keys(conn)?
            .into_iter()
            .filter_map(|foreign_key| foreign_key.foreign_table(conn).ok().flatten())
            .collect())
    }

    /// Returns the tables this table extends, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    ///
    /// # Implementation details
    ///
    /// A table is considered an extension table if it has a primary key
    /// composed of a single column which is also a foreign key to another
    /// table. This does apply to composite primary keys.
    pub fn ancestral_extension_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Table>, WebCodeGenError> {
        let mut tables = Vec::new();
        for extended_table in self.extension_tables(conn)? {
            for table in extended_table.ancestral_extension_tables(conn)? {
                if !tables.contains(&table) {
                    tables.push(table);
                }
            }
            tables.push(extended_table);
        }
        tables.sort_unstable();
        tables.dedup();
        Ok(tables)
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
            let mut primary_key_types = Vec::new();

            for column in primary_key_columns {
                let column_type = column.rust_data_type(conn)?;
                primary_key_types.push(column_type);
            }

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
    ) -> Result<Vec<Column>, diesel::result::Error> {
        primary_key_columns(self, conn)
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
    ) -> Result<Vec<CheckConstraint>, diesel::result::Error> {
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
    pub fn triggers(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<PgTrigger>, diesel::result::Error> {
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

    /// Returns whether the table must be inserted alongside ther `other` table
    /// in a composited builder.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Table`
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Implementation details
    ///
    /// A table must be inserted alongside `other` if:
    ///
    /// * The table inherits the `other` table or vice versa.
    /// * Either `A` or `B` table extends a third table `C`, (say `A`), and the
    ///   `B` table has a unique constraint on the primary key of `C`, making it
    ///   necessary to insert first `C`, then `B` and then `A`.
    fn _must_be_inserted_alongside_with(
        &self,
        other: &Self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let mut self_extension_tables = self.ancestral_extension_tables(conn)?;
        self_extension_tables.push(self.clone());
        self_extension_tables.sort_unstable();
        let mut other_extension_tables = other.ancestral_extension_tables(conn)?;
        other_extension_tables.push(other.clone());
        other_extension_tables.sort_unstable();

        // First simple case: if either table extends the other, we must insert them
        // alongside.
        if self_extension_tables.contains(other) || other_extension_tables.contains(self) {
            return Ok(true);
        }

        // Otherwise, we need to determine whether there are any foreign keys in `other`
        // or in the tables it extends that point to the primary key of any of the
        // tables extended by `A` (or `A` itself).
        //
        //   +---------+         +---------+
        //   |   C     |<--------|   B     |
        //   +---------+   FK    +---------+
        //        ^                 ^
        //        |                 |
        //   extends           extends
        //        |                 |
        //   +---------+       +---------+
        //   |   A     |       |  (other)|
        //   +---------+       +---------+
        //
        // Furthermore, the FK must also appear in a unique constraint defined in the
        // `B` table which contains the primary key of `B` and then the primary
        // key of `C`, therefore something like: `UNIQUE (b_id, c_id)`.
        //
        let other_foreign_keys: Vec<KeyColumnUsage> = other_extension_tables
            .iter()
            .flat_map(|other_extension_table| {
                other_extension_table
                    .same_as_indices(conn)
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|index| {
                        let foreign_key = index.is_same_as(conn).ok().flatten()?;
                        let foreign_table = foreign_key.foreign_table(conn).ok().flatten()?;
                        if self_extension_tables.binary_search(&foreign_table).is_ok() {
                            Some(foreign_key)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if other_foreign_keys.is_empty() {
            // If there are no foreign keys in `self` that point to the primary key of any
            // table that `other` extends, we can return false.
            return Ok(false);
        }

        // We collect all the tables which contain the definition of the
        // `other_foreign_keys`.
        let mut other_foreign_key_tables: Vec<Table> =
            other_foreign_keys.iter().map(|fk| fk.table(conn)).collect::<Result<Vec<_>, _>>()?;

        other_foreign_key_tables.sort_unstable();
        other_foreign_key_tables.dedup();

        // Next, having identified such a foreign key constraint, we check whether there
        // exist a same-as constraint between any of the tables extended by `A` (or `A`)
        // and the tables which contain the foreign key constraint in `B`.
        for self_extension_table in self_extension_tables {
            for column in self_extension_table.columns(conn)? {
                for same_as_constraint in column.same_as_constraints(conn)? {
                    let Some(foreign_table) = same_as_constraint.foreign_table(conn)? else {
                        continue;
                    };

                    if other_foreign_key_tables.binary_search(&foreign_table).is_ok() {
                        // If we found a same-as constraint between the table extended by `A`
                        // and the table containing the foreign key constraint in `B`, we must
                        // insert them alongside.
                        return Ok(true);
                    }
                }
            }
        }

        Ok(false)
    }

    /// Returns whether the table must be inserted alongside ther `other` table
    /// in a composited builder.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another `Table`
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Implementation details
    ///
    /// A table must be inserted alongside `other` if:
    ///
    /// * The table inherits the `other` table or vice versa.
    /// * Either `A` or `B` table extends a third table `C`, (say `A`), and the
    ///   `B` table has a unique constraint on the primary key of `C`, making it
    ///   necessary to insert first `C`, then `B` and then `A`.
    pub fn must_be_inserted_alongside_with(
        &self,
        other: &Self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self._must_be_inserted_alongside_with(other, conn)?
            || other._must_be_inserted_alongside_with(self, conn)?)
    }
}
