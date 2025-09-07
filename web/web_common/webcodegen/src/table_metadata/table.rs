use std::{collections::HashSet, fmt::Display, sync::Arc};

use cached::{DiskCache, proc_macro::io_cached};
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
    CheckConstraint, Column, PgIndex, TableConstraint, errors::WebCodeGenError,
    table_metadata::key_column_usage::PartialBuilderKind,
};

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.load_all_tables")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{}-{}", table_catalog, table_schema) }"#
)]
fn load_all_tables(
    table_catalog: &str,
    table_schema: &str,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Table>>, WebCodeGenError> {
    use crate::schema::tables;
    Ok(Arc::new(
        tables::table
            .filter(tables::table_catalog.eq(table_catalog))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_name.ne("__diesel_schema_migrations"))
            .order_by(tables::table_name)
            .select(Table::as_select())
            .load::<Table>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.load_table")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{}-{}-{}", table_name, table_schema, table_catalog) }"#
)]
fn load_table(
    conn: &mut PgConnection,
    table_name: &str,
    table_schema: &str,
    table_catalog: &str,
) -> Result<Arc<Table>, WebCodeGenError> {
    use crate::schema::tables;
    Ok(Arc::new(
        tables::table
            .filter(tables::table_name.eq(table_name))
            .filter(tables::table_schema.eq(table_schema))
            .filter(tables::table_catalog.eq(table_catalog))
            .first::<Table>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn columns(table: &Table, conn: &mut PgConnection) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
    use crate::schema::columns;
    Ok(Arc::new(
        columns::table
            .filter(columns::table_name.eq(&table.table_name))
            .filter(columns::table_schema.eq(&table.table_schema))
            .filter(columns::table_catalog.eq(&table.table_catalog))
            .order_by(columns::ordinal_position)
            .load::<Column>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.unique_columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn unique_columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<Vec<Column>>, WebCodeGenError> {
    use crate::schema::{columns, key_column_usage, table_constraints};
    Ok(key_column_usage::table
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
        })?)
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.primary_key_columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn primary_key_columns(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
    use crate::schema::{columns, key_column_usage, table_constraints};
    Ok(Arc::new(
        key_column_usage::table
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
            .filter(key_column_usage::table_name.eq(&table.table_name))
            .filter(key_column_usage::table_schema.eq(&table.table_schema))
            .filter(key_column_usage::table_catalog.eq(&table.table_catalog))
            .filter(table_constraints::constraint_type.eq("PRIMARY KEY"))
            .select(Column::as_select())
            .load::<Column>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.foreign_keys")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn foreign_keys(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<KeyColumnUsage>>, WebCodeGenError> {
    use crate::schema::{key_column_usage, referential_constraints};
    Ok(Arc::new(
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
            .load::<KeyColumnUsage>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.unique_indices")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn unique_indices(table: &Table, conn: &mut PgConnection) -> Result<Vec<PgIndex>, WebCodeGenError> {
    use crate::schema::{pg_class, pg_index};

    let (pg_class1, pg_class2) = diesel::alias!(pg_class as pg_class1, pg_class as pg_class2);

    Ok(pg_index::table
        .inner_join(pg_class1.on(pg_class1.field(pg_class::oid).eq(pg_index::indexrelid)))
        .inner_join(pg_class2.on(pg_class2.field(pg_class::oid).eq(pg_index::indrelid)))
        .filter(pg_class2.field(pg_class::relname).eq(&table.table_name).and(
            pg_class2.field(pg_class::relnamespace).eq(pg_class1.field(pg_class::relnamespace)),
        ))
        .filter(pg_index::indisunique.eq(true))
        .select(PgIndex::as_select())
        .load::<PgIndex>(conn)?)
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.same_as_indices")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn same_as_indices(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Vec<PgIndex>, WebCodeGenError> {
    Ok(unique_indices(table, conn)?
        .into_iter()
        .filter(|pg_index| pg_index.is_same_as(conn).unwrap_or(false))
        .collect())
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.ancestral_extension_tables")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn ancestral_extension_tables(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Table>>, WebCodeGenError> {
    let mut tables = Vec::new();
    for extended_table in table.extension_tables(conn)?.as_ref() {
        tables
            .extend(extended_table.ancestral_extension_tables(conn)?.as_ref().into_iter().cloned());
        tables.push(extended_table.as_ref().clone());
    }
    tables.sort_unstable();
    tables.dedup();
    Ok(Arc::new(tables))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("table.extension_tables")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ table.to_string() }"#
)]
fn extension_tables(
    table: &Table,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Arc<Table>>>, WebCodeGenError> {
    Ok(Arc::new(
        table
            .extension_foreign_keys(conn)?
            .into_iter()
            .filter_map(|foreign_key| foreign_key.foreign_table(conn).ok())
            .collect(),
    ))
}

#[derive(
    Queryable,
    QueryableByName,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Selectable,
    Debug,
    Clone,
    Hash,
    serde::Serialize,
    serde::Deserialize,
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

impl Display for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}.{}`", self.table_schema, self.table_name)
    }
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

    /// Returns wether a table require authorizations to be viewed
    pub fn require_view_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns wether a table require authorizations to be modified
    pub fn require_modify_authorizations(&self, _conn: &mut PgConnection) -> bool {
        false
    }

    /// Returns whether the table has a single-column primary key which
    /// is a UUID generated by the `uuid_generate_v4()` function. If it
    /// does, it returns the column, None otherwise.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If the database connection fails.
    pub(crate) fn has_uuid_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, WebCodeGenError> {
        let primary_key_columns = self.primary_key_columns(conn)?;
        if primary_key_columns.len() != 1 {
            return Ok(None);
        }
        let primary_key_column = primary_key_columns.as_ref()[0].clone();
        if !primary_key_column.is_uuid() {
            return Ok(None);
        }
        if primary_key_column.has_uuid_generator() {
            return Ok(Some(primary_key_column));
        }
        Ok(None)
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
    pub fn primary_key_idents(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Ident>, WebCodeGenError> {
        self.primary_key_columns(conn)?
            .as_ref()
            .iter()
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
            let primary_key_idents = self.primary_key_idents(conn)?;
            quote! {
                #[diesel(primary_key(#(#primary_key_idents),*))]
            }
        } else {
            TokenStream::new()
        })
    }

    /// Returns whether the table has singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn has_singleton_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.foreign_keys(conn)?.iter().any(|fk| fk.is_singleton(conn).unwrap_or(false)))
    }

    /// Returns the table singleton foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the foreign keys cannot be loaded from the database.
    pub fn singleton_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .filter(|fk| fk.is_singleton(conn).unwrap_or(false))
            .cloned()
            .collect())
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

        if self.has_singleton_foreign_keys(conn)? {
            derives.push(parse_str("diesel::Associations")?);
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
    ) -> Result<Arc<Vec<KeyColumnUsage>>, WebCodeGenError> {
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
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if foreign_key.has_on_delete_cascade(conn)?
                && foreign_key.is_foreign_primary_key(conn)?
                && !foreign_key.is_self_referential(conn)?
            {
                parent_keys.push(foreign_key.clone());
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
    pub fn parent_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut parent_tables = Vec::new();
        for foreign_key in self.parent_keys(conn)? {
            parent_tables.push(foreign_key.foreign_table(conn)?);
        }
        parent_tables.sort_unstable();
        parent_tables.dedup();
        Ok(parent_tables)
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
    ) -> Result<Vec<Arc<Vec<Column>>>, WebCodeGenError> {
        let mut homogeneous_parent_columns = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if foreign_table.as_ref() == self && foreign_key.is_foreign_primary_key(conn)? {
                homogeneous_parent_columns.push(foreign_key.columns(conn)?);
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
    pub fn foreign_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut tables = HashSet::new();
        for foreign_key_constraint in self.foreign_keys(conn)?.as_ref() {
            tables.insert(foreign_key_constraint.foreign_table(conn)?);
        }
        let mut tables = tables.into_iter().collect::<Vec<Arc<Table>>>();
        tables.sort_unstable();
        Ok(tables)
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
        for column in self.columns(conn)?.as_ref() {
            if column.is_created_by(conn)? {
                return Ok(true);
            }
        }

        if include_ancestors {
            for table in self.extension_tables(conn)?.as_ref() {
                if table.has_created_by_column(true, conn)? {
                    return Ok(true);
                }
            }
        }

        Ok(false)
    }

    /// Returns the most concrete table column, if any.
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
    pub fn most_concrete_table_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if column.is_most_concrete_table(conn)? {
                return Ok(Some(column.clone()));
            }
        }

        if include_ancestors {
            let extension_tables = self.extension_tables(conn)?;
            for table in extension_tables.as_ref() {
                if let Some(column) = table.most_concrete_table_column(true, conn)? {
                    return Ok(Some(column));
                }
            }

            // All tables in the extension hierarchy must have the
            // most concrete table column in their root.
            if !extension_tables.is_empty() {
                unreachable!(
                    "The current SQL schema is invalid: the extension hierarchy of table {self} does not have a `most concrete table` column.",
                )
            }
        }

        Ok(None)
    }

    /// Returns whether the table has a `most concrete table` column.
    ///
    /// # Arguments
    ///
    /// * `include_ancestors` - Whether to include ancestor tables in the
    ///  search.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * Returns an error if the provided database connection is invalid.
    pub fn has_most_concrete_table_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.most_concrete_table_column(include_ancestors, conn)?.is_some())
    }

    /// Returns the `updated_by` column of the table, if any.
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
    pub(crate) fn updated_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<Column>, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if column.is_updated_by(conn)? {
                return Ok(Some(column.clone()));
            }
        }
        if include_ancestors {
            for table in self.extension_tables(conn)?.as_ref() {
                if let Some(column) = table.updated_by_column(true, conn)? {
                    return Ok(Some(column));
                }
            }
        }
        Ok(None)
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
    pub(crate) fn has_updated_by_column(
        &self,
        include_ancestors: bool,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.updated_by_column(include_ancestors, conn)?.is_some())
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
    pub fn unique_indices(&self, conn: &mut PgConnection) -> Result<Vec<PgIndex>, WebCodeGenError> {
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
    ) -> Result<Vec<PgIndex>, WebCodeGenError> {
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
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if let Some(index) = foreign_key.is_same_as_constraint(conn)? {
                same_as_foreign_keys.push((foreign_key.clone(), index));
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
        table_schema: &str,
    ) -> Result<Arc<Vec<Self>>, WebCodeGenError> {
        load_all_tables(table_catalog, table_schema, conn)
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
        for column in self.columns(conn)?.as_ref() {
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
        for column in self.columns(conn)?.as_ref() {
            if !column.supports_eq(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    /// Returns whether the table supports the `PartialOrd` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If database connection fails.
    pub fn supports_partial_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        for column in self.columns(conn)?.as_ref() {
            if !column.supports_partial_ord(conn)? {
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
        for column in self.columns(conn)?.as_ref() {
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
        for column in self.columns(conn)?.as_ref() {
            if !column.supports_hash(conn)? {
                return Ok(false);
            }
        }
        Ok(true)
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
        table_schema: &str,
        table_catalog: &str,
    ) -> Result<Arc<Self>, WebCodeGenError> {
        load_table(conn, table_name, table_schema, table_catalog)
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
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
        columns(self, conn)
    }

    /// Returns the columns and foreign keys of the table which require partial
    /// builders.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the columns cannot be loaded from the database.
    pub fn partial_builder_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, PartialBuilderKind, KeyColumnUsage, KeyColumnUsage)>, WebCodeGenError>
    {
        Ok(self
            .columns(conn)?
            .as_ref()
            .iter()
            .filter_map(|column| {
                let (partial_builder_kind, potential_same_as_constraint, foreign_key) =
                    column.requires_partial_builder(conn).ok().flatten()?;
                Some((
                    column.clone(),
                    partial_builder_kind,
                    potential_same_as_constraint,
                    foreign_key,
                ))
            })
            .collect())
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
    ) -> Result<Column, WebCodeGenError> {
        use crate::schema::columns;
        Ok(columns::table
            .filter(columns::table_name.eq(&self.table_name))
            .filter(columns::table_schema.eq(&self.table_schema))
            .filter(columns::table_catalog.eq(&self.table_catalog))
            .filter(columns::column_name.eq(column_name))
            .first::<Column>(conn)?)
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
    pub fn has_primary_keys(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.primary_key_columns(conn).map(|columns| !columns.is_empty())
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
            .as_ref()
            .iter()
            .filter(|foreign_key| foreign_key.is_extension(conn).unwrap_or(false))
            .cloned()
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
    pub fn extension_tables(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<Arc<Table>>>, WebCodeGenError> {
        extension_tables(self, conn)
    }

    /// Returns whether the current table extends any other table.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the database connection is invalid.
    pub fn is_extension(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(!self.extension_tables(conn)?.is_empty())
    }

    /// Returns the associated same-as foreign keys of the table, if any.
    ///
    /// # Arguments
    ///
    /// * `include_local_primary_key` - Whether to include the local primary key
    ///   in the constraint.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn associated_same_as_foreign_keys(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut associated_foreign_key = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if foreign_key
                .is_associated_same_as_constraint(include_local_primary_key, conn)?
                .is_some()
            {
                associated_foreign_key.push(foreign_key.clone());
            }
        }

        Ok(associated_foreign_key)
    }

    /// Returns the ancestral same-as foreign keys of the table, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn ancestral_same_as_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut ancestral_foreign_key = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            if foreign_key.is_ancestral_same_as_constraint(conn)?.is_some() {
                ancestral_foreign_key.push(foreign_key.clone());
            }
        }

        ancestral_foreign_key.sort_unstable();
        ancestral_foreign_key.dedup();

        Ok(ancestral_foreign_key)
    }

    /// Returns the associated tables this table references via foreign keys, if
    /// any.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Implementative details
    ///
    /// A table referenced by another table is considered associated if any
    /// columns of the latter table referencing the former table are not
    /// part of the primary key of the latter table, and still require a
    /// partial builder.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub(crate) fn associated_tables(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut associated_tables = Vec::new();
        for foreign_key in self.associated_same_as_foreign_keys(include_local_primary_key, conn)? {
            let foreign_table = foreign_key.foreign_table(conn)?;
            associated_tables.push(foreign_table);
        }

        associated_tables.sort_unstable();
        associated_tables.dedup();

        Ok(associated_tables)
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
    ) -> Result<Arc<Vec<Table>>, WebCodeGenError> {
        ancestral_extension_tables(self, conn)
    }

    /// Returns whether the table is extending the provided table.
    ///
    /// # Arguments
    ///
    /// * `ancestor` - The potential ancestor table.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn is_extending(
        &self,
        ancestor: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        self.ancestral_extension_tables(conn).map(|tables| tables.contains(ancestor))
    }

    /// Returns whether the table shares any ancestor with the provided table.
    ///
    /// # Arguments
    ///
    /// * `other` - The other table.
    /// * `conn` - The database connection.
    ///
    /// # Errors
    ///
    /// * If the table cannot be loaded from the database.
    pub fn share_ancestors(
        &self,
        other: &Table,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let self_ancestors = self.ancestral_extension_tables(conn)?;
        let other_ancestors = other.ancestral_extension_tables(conn)?;

        for ancestor in self_ancestors.as_ref() {
            if other_ancestors.contains(ancestor) {
                return Ok(true);
            }
        }

        Ok(false)
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

            for column in primary_key_columns.as_ref() {
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
        let mut columns = self.columns(conn)?.as_ref().clone();
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
    ) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
        primary_key_columns(self, conn)
    }

    /// Returns whether the table has a composite primary key.
    ///
    /// # Arguments
    ///
    /// * `conn` - The database connection.
    ///
    /// # Returns
    ///
    /// A boolean indicating whether the table has a composite primary key.
    ///
    /// # Errors
    ///
    /// * If the primary key columns cannot be loaded from the database.
    pub fn has_composite_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.primary_key_columns(conn)?.len() > 1)
    }

    /// Returns the subset of the table's columns which define other
    /// columns' values via foreign key constraints.
    pub(crate) fn foreign_definer_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut foreign_definer_columns = Vec::new();
        for column in self.columns(conn)?.as_ref() {
            if column.is_foreign_definer(conn)? {
                foreign_definer_columns.push(column.clone());
            }
        }
        Ok(foreign_definer_columns)
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
    ) -> Result<Vec<CheckConstraint>, WebCodeGenError> {
        use crate::schema::{check_constraints, table_constraints};

        Ok(check_constraints::table
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
            .load::<CheckConstraint>(conn)?)
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
    pub fn triggers(&self, conn: &mut PgConnection) -> Result<Vec<PgTrigger>, WebCodeGenError> {
        use crate::schema::{pg_class, pg_namespace, pg_trigger};
        Ok(pg_trigger::table
            .inner_join(pg_class::table.on(pg_trigger::tgrelid.eq(pg_class::oid)))
            .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
            .filter(pg_class::relname.eq(&self.table_name))
            .filter(pg_namespace::nspname.eq(&self.table_schema))
            .select(PgTrigger::as_select())
            .load::<PgTrigger>(conn)?)
    }
}

impl AsRef<Table> for Table {
    fn as_ref(&self) -> &Table {
        self
    }
}
