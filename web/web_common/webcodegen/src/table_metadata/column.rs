use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::Arc,
};

use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
};
use proc_macro2::TokenStream;
use syn::{Ident, Type};

mod default_types;
use cached::{
    DiskCache,
    proc_macro::{cached, io_cached},
};
pub use default_types::DefaultTypes;

use super::{
    check_constraint::CheckConstraint,
    pg_type::{COPY_TYPES, EQ_TYPES, HASH_TYPES, ORD_TYPES, PgType, rust_type_str},
};
use crate::{
    KeyColumnUsage, Table,
    errors::WebCodeGenError,
    table_metadata::{
        key_column_usage::PartialBuilderKind,
        pg_type::{PARTIAL_ORD_TYPES, postgres_type_to_diesel},
    },
    utils::{RESERVED_DIESEL_WORDS, RESERVED_RUST_WORDS},
};

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("column.foreign_keys")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{column}") }"#
)]
fn foreign_keys(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<KeyColumnUsage>>, WebCodeGenError> {
    use crate::schema::{
        columns, constraint_column_usage, key_column_usage, table_constraints, tables,
    };
    Ok(Arc::new(
        table_constraints::table
            .inner_join(
                key_column_usage::table.on(table_constraints::constraint_name
                    .eq(key_column_usage::constraint_name)
                    .and(
                        table_constraints::constraint_schema
                            .eq(key_column_usage::constraint_schema),
                    )
                    .and(
                        table_constraints::constraint_catalog
                            .eq(key_column_usage::constraint_catalog),
                    )
                    .and(table_constraints::table_name.eq(key_column_usage::table_name))
                    .and(table_constraints::table_schema.eq(key_column_usage::table_schema))
                    .and(table_constraints::table_catalog.eq(key_column_usage::table_catalog))),
            )
            .inner_join(constraint_column_usage::table.on(
                constraint_column_usage::constraint_name.eq(table_constraints::constraint_name),
            ))
            .inner_join(
                tables::table.on(tables::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(tables::table_schema.eq(constraint_column_usage::table_schema))
                    .and(tables::table_catalog.eq(constraint_column_usage::table_catalog))),
            )
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                    .and(columns::table_catalog.eq(constraint_column_usage::table_catalog))
                    .and(columns::column_name.eq(constraint_column_usage::column_name))),
            )
            .filter(table_constraints::constraint_type.eq("FOREIGN KEY"))
            .filter(table_constraints::table_name.eq(&column.table_name))
            .filter(table_constraints::table_schema.eq(&column.table_schema))
            .filter(table_constraints::table_catalog.eq(&column.table_catalog))
            .filter(key_column_usage::column_name.eq(&column.column_name))
            .order_by(key_column_usage::constraint_name.asc())
            .select(KeyColumnUsage::as_select())
            .distinct()
            .load::<KeyColumnUsage>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("column.check_constraints")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{column}") }"#
)]
fn check_constraints(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<CheckConstraint>>, WebCodeGenError> {
    use crate::schema::{check_constraints, constraint_column_usage};
    Ok(Arc::new(
        check_constraints::table
            .inner_join(
                constraint_column_usage::table.on(constraint_column_usage::constraint_name
                    .eq(check_constraints::constraint_name)
                    .and(
                        constraint_column_usage::constraint_catalog
                            .eq(check_constraints::constraint_catalog)
                            .and(
                                constraint_column_usage::constraint_schema
                                    .eq(check_constraints::constraint_schema),
                            ),
                    )),
            )
            .filter(
                constraint_column_usage::column_name.eq(&column.column_name).and(
                    constraint_column_usage::table_catalog.eq(&column.table_catalog).and(
                        constraint_column_usage::table_schema
                            .eq(&column.table_schema)
                            .and(constraint_column_usage::table_name.eq(&column.table_name)),
                    ),
                ),
            )
            .select(CheckConstraint::as_select())
            .load(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("column.table")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{column}") }"#
)]
fn table(column: &Column, conn: &mut PgConnection) -> Result<Arc<Table>, WebCodeGenError> {
    use crate::schema::tables;
    Ok(Arc::new(
        tables::table
            .filter(tables::table_name.eq(&column.table_name))
            .filter(tables::table_schema.eq(&column.table_schema))
            .filter(tables::table_catalog.eq(&column.table_catalog))
            .first::<Table>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("column.geometry")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{column}") }"#
)]
fn geometry(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Option<crate::GeometryColumn>, WebCodeGenError> {
    use crate::schema::geometry_columns;

    Ok(geometry_columns::table
        .filter(geometry_columns::f_table_name.eq(&column.table_name))
        .filter(geometry_columns::f_table_schema.eq(&column.table_schema))
        .filter(geometry_columns::f_geometry_column.eq(&column.column_name))
        .first::<crate::GeometryColumn>(conn)
        .optional()?)
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("column.geography")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "String",
    convert = r#"{ format!("{column}") }"#
)]
fn geography(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Option<crate::GeographyColumn>, WebCodeGenError> {
    use crate::schema::geography_columns;

    Ok(geography_columns::table
        .filter(geography_columns::f_table_name.eq(&column.table_name))
        .filter(geography_columns::f_table_schema.eq(&column.table_schema))
        .filter(geography_columns::f_geography_column.eq(&column.column_name))
        .first::<crate::GeographyColumn>(conn)
        .optional()?)
}

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn str_rust_data_type(column: &Column, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
    if let Ok(Some(geometry)) = column.geometry(conn) {
        return Ok(geometry.str_rust_type().to_owned());
    }
    if let Ok(Some(geography)) = column.geography(conn) {
        return Ok(geography.str_rust_type().to_owned());
    }
    match rust_type_str(column.data_type_str(conn)?, conn) {
        Ok(s) => Ok(s.to_string()),
        Err(error) => {
            if column.has_custom_type() {
                Ok(PgType::from_name(column.data_type_str(conn)?, conn)?.camelcased_name())
            } else {
                Err(error)
            }
        }
    }
}

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn ancestral_same_as_reachable_set(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<HashSet<Column>, WebCodeGenError> {
    let ancestral_same_as_columns = column.ancestral_same_as_columns(conn)?;
    let mut reachable_set = ancestral_same_as_columns.iter().cloned().collect::<HashSet<_>>();
    for column in &ancestral_same_as_columns {
        let ancestral_same_as_columns = column.ancestral_same_as_reachable_set(conn)?;
        reachable_set.extend(ancestral_same_as_columns);
    }
    Ok(reachable_set)
}

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn associated_same_as_columns(
    column: &Column,
    include_local_primary_key: bool,
    conn: &mut PgConnection,
) -> Result<Vec<(Column, KeyColumnUsage)>, WebCodeGenError> {
    column
        .associated_same_as_constraints(include_local_primary_key, conn)?
        .into_iter()
        .map(|constraint| {
            let local_columns = constraint.columns(conn)?;
            let foreign_columns = constraint.foreign_columns(conn)?;
            Ok((
                local_columns
                    .iter()
                    .zip(foreign_columns.iter())
                    .filter_map(|(local_column, foreign_column)| {
                        if local_column == column { Some(foreign_column.clone()) } else { None }
                    })
                    .next()
                    .unwrap(),
                constraint,
            ))
        })
        .collect()
}

#[cached(result = true, key = "String", convert = r#"{ format!("{column}") }"#)]
fn all_ancestral_same_as_columns(
    column: &Column,
    conn: &mut PgConnection,
) -> Result<Vec<Column>, WebCodeGenError> {
    let mut reachable_set = column.ancestral_same_as_reachable_set(conn)?;
    // The frontier contains the set of columns which so far can only be reached
    // from the current column. Once a column in the frontier is found to be
    // reachable from another column in the reachable set, it is marked as true
    // and will not be used to expand the reachable set anymore.
    let mut frontier: HashMap<Column, bool> =
        column.ancestral_same_as_columns(conn)?.into_iter().map(|c| (c, false)).collect();
    let table = column.table(conn)?;
    let ancestral_extension_tables = table.ancestral_extension_tables(conn)?;
    let mut changed = true;

    while changed {
        changed = false;
        for ancestor in ancestral_extension_tables.iter() {
            for ancestor_column in ancestor.columns(conn)?.iter() {
                // If the ancestor node is already in the reachable set, skip it.
                if reachable_set.contains(&ancestor_column) {
                    continue;
                }

                let ancestor_reachable_set =
                    ancestor_column.ancestral_same_as_reachable_set(conn)?;

                // We update the frontier to mark as true columns which we have now discovered
                // can be reached also from this ancestor column.
                for (frontier_column, is_reachable) in frontier.iter_mut() {
                    if !*is_reachable && ancestor_reachable_set.contains(frontier_column) {
                        *is_reachable = true;
                        changed = true;
                    }
                }

                // If the ancestor reachable set intersects with the current reachable
                // set, then the ancestor column is inferred to be ancestrally same-as
                // the current column. We then merge the ancestor reachable set into
                // the current reachable set, and add the ancestor column to the
                // frontier.
                if !reachable_set.is_disjoint(&ancestor_reachable_set) {
                    reachable_set.insert(ancestor_column.clone());
                    frontier.insert(ancestor_column.clone(), false);
                    reachable_set.extend(ancestor_reachable_set);
                    changed = true;
                }
            }
        }
    }

    // We then consider as ancestrally same-as only those columns in the frontier
    // which are still marked as false, meaning they could not be reached
    // from any other column in the reachable set.
    let mut ancestral_same_as_columns = frontier
        .into_iter()
        .filter_map(|(column, is_reachable)| if !is_reachable { Some(column) } else { None })
        .collect::<Vec<_>>();
    ancestral_same_as_columns.sort_unstable();

    Ok(ancestral_same_as_columns)
}

/// Struct defining the `information_schema.columns` table.
#[derive(
    Queryable,
    QueryableByName,
    Selectable,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Debug,
    Clone,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(table_name = crate::schema::columns)]
pub struct Column {
    /// Name of the database containing the table (always the current database)
    pub table_catalog: String,
    /// Name of the schema containing the table
    pub table_schema: String,
    /// Name of the table containing the column
    pub table_name: String,
    /// Name of the column
    pub column_name: String,
    /// Ordinal position of the column within the table (co nt starts at 1)
    pub ordinal_position: i32,
    /// Default expression of the column
    pub column_default: Option<String>,
    /// Indicates if the column is nullable ("YES" or "NO")
    __is_nullable: String,
    /// Data type of the column
    data_type: String,
    /// Maximum length of the character data type
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes of the character data type
    pub character_octet_length: Option<i32>,
    /// Precision of the numeric data type
    pub numeric_precision: Option<i32>,
    /// Radix (base) of the numeric data type
    pub numeric_precision_radix: Option<i32>,
    /// Scale of the numeric data type
    pub numeric_scale: Option<i32>,
    /// Precision of the datetime data type
    pub datetime_precision: Option<i32>,
    /// Interval type of the interval data type
    pub interval_type: Option<String>,
    /// Precision of the interval data type
    pub interval_precision: Option<i32>,
    /// Catalog name of the underlying type of the column
    pub udt_catalog: Option<String>,
    /// Schema name of the underlying type of the column
    pub udt_schema: Option<String>,
    /// Name of the underlying type of the column
    pub udt_name: Option<String>,
    /// Catalog name of the scope of the column
    pub scope_catalog: Option<String>,
    /// Schema name of the scope of the column
    pub scope_schema: Option<String>,
    /// Name of the scope of the column
    pub scope_name: Option<String>,
    /// Maximum cardinality of the column
    pub maximum_cardinality: Option<i32>,
    /// Identifier of the data type descriptor
    pub dtd_identifier: Option<String>,
    /// Indicates if the column is self-referencing
    pub is_self_referencing: Option<String>,
    /// Indicates if the column is an identity column
    pub is_identity: Option<String>,
    /// Indicates if the column is generated ("ALWAYS" or "NEVER")
    pub is_generated: String,
    /// Generation expression of the column
    pub generation_expression: Option<String>,
    /// Indicates if the column is updatable ("YES" or "NO")
    pub is_updatable: String,
}

impl Display for Column {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`{}.{}.{}`", self.table_schema, self.table_name, self.column_name)
    }
}

impl AsRef<Column> for Column {
    fn as_ref(&self) -> &Column {
        self
    }
}

impl Column {
    #[must_use]
    /// Returns the column as a nullable column
    pub fn into_nullable(self) -> Self {
        Self { __is_nullable: "YES".to_string(), ..self }
    }

    #[must_use]
    /// Returns the column as a nullable column
    pub fn to_nullable(&self) -> Self {
        self.clone().into_nullable()
    }

    #[must_use]
    /// Returns the column as a non-nullable column
    pub fn into_non_nullable(self) -> Self {
        Self { __is_nullable: "NO".to_string(), ..self }
    }

    #[must_use]
    /// Returns the column as a non-nullable column
    pub fn to_non_nullable(&self) -> Self {
        self.clone().into_non_nullable()
    }

    #[must_use]
    /// Returns the raw data type of the column
    pub fn raw_data_type(&self) -> &str {
        &self.data_type
    }

    /// Returns whether the column contains `PostGIS` geometry data
    pub fn is_geometry(&self, conn: &mut PgConnection) -> bool {
        self.geometry(conn).ok().flatten().is_some()
    }

    /// Returns all the check constraint associated to the current column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If their is an error while querying the [`CheckConstraint`].
    ///
    /// # Returns
    ///
    /// * A `Vec` of all the [`CheckConstraint`]
    pub fn check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<CheckConstraint>>, WebCodeGenError> {
        check_constraints(self, conn)
    }

    /// Returns the associated geometry column if the column is a geometry
    /// column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn geometry(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<crate::GeometryColumn>, WebCodeGenError> {
        geometry(self, conn)
    }

    /// Returns the associated geography column if the column is a geography
    /// column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn geography(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<crate::GeographyColumn>, WebCodeGenError> {
        geography(self, conn)
    }

    /// Returns the data type associated with the column as repo
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the data type of the column if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn data_type_str(&self, _conn: &mut PgConnection) -> Result<&str, WebCodeGenError> {
        Ok(if self.has_custom_type() {
            if let Some(udt_name) = &self.udt_name {
                udt_name
            } else {
                return Err(WebCodeGenError::UnknownColumnType(Box::new(self.clone())));
            }
        } else {
            &self.data_type
        })
    }

    /// Returns the [`PgType`] associated with the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the [`PgType`] of the column if the operation was
    /// successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn pg_type(&self, conn: &mut PgConnection) -> Result<PgType, WebCodeGenError> {
        use crate::schema::{pg_attribute, pg_class, pg_namespace, pg_type};

        Ok(pg_type::table
            .inner_join(pg_attribute::table.on(pg_attribute::atttypid.eq(pg_type::oid)))
            .inner_join(pg_class::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
            .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
            .filter(pg_class::relname.eq(&self.table_name))
            .filter(pg_namespace::nspname.eq(&self.table_schema))
            .filter(pg_attribute::attname.eq(&self.column_name))
            .select(PgType::as_select())
            .first::<PgType>(conn)?)
    }

    /// Returns the string data type
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn str_rust_data_type(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        str_rust_data_type(self, conn)
    }

    /// Returns whether the column is compatible with the provided column
    ///
    /// # Arguments
    ///
    /// * `column` - A reference to a `Column`
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    /// * If the underlying data type of the column is not compatible
    ///
    /// # Implementative details
    ///
    /// The two columns are considered compatible if their data type is the
    /// same, and if they have shared ancestors.
    pub fn has_compatible_data_type(
        &self,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        if self.str_rust_data_type(conn)? != column.str_rust_data_type(conn)? {
            return Ok(false);
        }

        let mut local_referenced_tables: Vec<Table> = self
            .referenced_by_foreign_keys(conn)?
            .into_iter()
            .map(|table| table.as_ref().clone())
            .collect::<Vec<_>>();
        let mut other_referenced_tables: Vec<Table> = column
            .referenced_by_foreign_keys(conn)?
            .into_iter()
            .map(|table| table.as_ref().clone())
            .collect::<Vec<_>>();

        if local_referenced_tables.is_empty() && other_referenced_tables.is_empty() {
            // If both columns are not foreign keys, they are compatible.
            return Ok(true);
        }

        // We determine the set of ancestors of the referenced tables.
        let local_referenced_ancestors = local_referenced_tables
            .iter()
            .flat_map(|table| {
                table.ancestral_extension_tables(conn).unwrap_or_default().as_ref().clone()
            })
            .collect::<Vec<_>>();
        let other_referenced_ancestors = other_referenced_tables
            .iter()
            .flat_map(|table| {
                table.ancestral_extension_tables(conn).unwrap_or_default().as_ref().clone()
            })
            .collect::<Vec<_>>();

        // We extend the referenced tables with their ancestors.
        local_referenced_tables.extend(local_referenced_ancestors);
        other_referenced_tables.extend(other_referenced_ancestors);

        Ok(local_referenced_tables.iter().any(|table| other_referenced_tables.contains(table)))
    }

    /// Returns the rust type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the rust `Type` of the column if the operation
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn rust_data_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        if let Ok(Some(geometry)) = self.geometry(conn) {
            return geometry.rust_type(self.is_nullable());
        }
        if let Ok(Some(geography)) = self.geography(conn) {
            return geography.rust_type(self.is_nullable());
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => {
                if self.is_nullable() {
                    Ok(syn::parse_str(&format!("Option<{s}>"))?)
                } else {
                    Ok(syn::parse_str(s)?)
                }
            }
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?
                        .rust_type(self.is_nullable(), conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns the rust reference type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the rust `Type` of the column if the operation
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn rust_ref_data_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        let rust_type = match self.str_rust_data_type(conn)?.as_str() {
            "String" => "&str".to_owned(),
            "Vec<u8>" => "&[u8]".to_owned(),
            other => format!("&{other}"),
        };

        let rust_type =
            if self.is_nullable() { format!("Option<{rust_type}>") } else { rust_type.to_string() };

        Ok(syn::parse_str(&rust_type)?)
    }

    /// Returns whether the column name is a reserved diesel word.
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn requires_diesel_sanitization(&self) -> Result<bool, WebCodeGenError> {
        Ok(RESERVED_DIESEL_WORDS.contains(&self.snake_case_name()?.as_str()))
    }

    /// Returns the sanitized snake case name of the table.
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn snake_case_name(&self) -> Result<String, WebCodeGenError> {
        crate::utils::snake_case_name(&self.column_name)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    ///
    /// If the column name is a reserved diesel word, the returned ident will be
    /// prefixed with `__`. If the column name is a reserved rust word, the
    /// returned ident will be the raw ident. Otherwise, the returned ident
    /// will be the sanitized snake case ident.
    ///
    /// # Returns
    ///
    /// A `Result` containing the sanitized snake case `Ident` if the operation
    /// was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while sanitizing the column name
    pub fn snake_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if self.requires_diesel_sanitization()? {
            Ok(Ident::new(&format!("__{snake_case_name}"), proc_macro2::Span::call_site()))
        } else if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(&snake_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the sanitized camel case name of the table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    pub fn camel_case_name(&self) -> Result<String, WebCodeGenError> {
        crate::utils::camel_case_name(&self.column_name)
    }

    /// Returns the sanitized camel case syn Ident of the table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    pub fn camel_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let camel_case_name = self.camel_case_name()?;
        if RESERVED_RUST_WORDS.contains(&camel_case_name.as_str()) {
            Ok(Ident::new_raw(&camel_case_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&camel_case_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the uppercased acronym of the column name.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the acronym
    pub fn acronym(&self) -> Result<String, WebCodeGenError> {
        let camel_cased_name = self.snake_case_name()?;
        Ok(camel_cased_name
            .split('_')
            .filter_map(|s| s.chars().next())
            .map(|c| c.to_ascii_uppercase())
            .collect())
    }

    /// Returns the uppercased acronym syn Ident of the column name.
    ///
    /// # Errors
    ///
    /// * If an error occurs while generating the acronym
    pub fn acronym_ident(&self) -> Result<Ident, WebCodeGenError> {
        let acronym = self.acronym()?;
        if RESERVED_RUST_WORDS.contains(&acronym.as_str()) {
            Ok(Ident::new_raw(&acronym, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&acronym, proc_macro2::Span::call_site()))
        }
    }

    #[must_use]
    /// Returns whether the column has a custom type
    pub fn has_custom_type(&self) -> bool {
        self.data_type == "USER-DEFINED"
    }

    #[must_use]
    /// Returns whether the column is nullable
    pub fn is_nullable(&self) -> bool {
        self.__is_nullable == "YES"
    }

    /// Returns the table which contains the current column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn table(&self, conn: &mut PgConnection) -> Result<Arc<Table>, WebCodeGenError> {
        table(self, conn)
    }

    /// Returns whether the column is part of a single-column unique constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_unique(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let table = self.table(conn)?;
        let pg_indices = table.unique_indices(conn)?;

        for index in pg_indices {
            let Ok(columns) = index.columns(conn) else {
                return Ok(false);
            };
            if columns.len() == 1 && columns[0].column_name == self.column_name {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Returns whether the column type implements copy.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_copy(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if let Ok(Some(geometry)) = self.geometry(conn) {
            return Ok(geometry.supports_copy());
        }
        if let Ok(Some(geography)) = self.geography(conn) {
            return Ok(geography.supports_copy());
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(COPY_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_copy(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Hash` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_hash(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(HASH_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_hash(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Eq` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_eq(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(EQ_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_eq(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `PartialOrd` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_partial_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.supports_ord(conn)? {
            return Ok(true);
        }
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(PARTIAL_ORD_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?
                        .supports_partial_ord(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns whether the column type supports the `Ord` trait.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn supports_ord(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.geometry(conn).ok().flatten().is_some()
            || self.geography(conn).ok().flatten().is_some()
        {
            return Ok(false);
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(ORD_TYPES.contains(&s)),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.supports_ord(conn)?)
                } else {
                    Err(error)
                }
            }
        }
    }

    /// Returns the diesel type of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing the diesel `Type` of the column if the operation
    /// was successful, or a `WebCodeGenError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn diesel_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        let tentative_type = postgres_type_to_diesel(self.data_type_str(conn)?, self.is_nullable());
        match tentative_type {
            Ok(t) => Ok(t),
            Err(e) => {
                if self.has_custom_type() {
                    PgType::from_name(self.data_type_str(conn)?, conn)?
                        .diesel_type(self.is_nullable(), conn)
                } else {
                    Err(e)
                }
            }
        }
    }

    #[must_use]
    /// Returns whether the column is a UUID
    pub fn is_uuid(&self) -> bool {
        self.data_type == "uuid"
    }

    #[must_use]
    /// Returns whether the column has a uuid-generator.
    pub fn has_uuid_generator(&self) -> bool {
        self.column_default.as_ref().is_some_and(|d| d == "gen_random_uuid()")
    }

    #[must_use]
    /// Returns whether the column is automatically generated
    ///
    /// A column is automatically generated if:
    /// - it is marked as `ALWAYS` generated
    /// - it has a default value that starts with `nextval`
    /// - it has a default value that starts with `CURRENT_TIMESTAMP`
    /// - it is an identity column
    /// - it is the extension primary key and its referenced primary key column
    ///   is automatically generated.
    pub fn is_always_automatically_generated(&self) -> bool {
        self.is_generated == "ALWAYS"
            || self.column_default.as_ref().is_some_and(|d| d.starts_with("nextval"))
            || self.is_identity.as_ref().is_some_and(|i| i == "YES")
    }

    #[must_use]
    /// Returns whether the current column has a DEFAULT value
    pub fn has_default(&self) -> bool {
        self.column_default.is_some()
    }

    /// Returns the rust `TokenStream` to create the default value of the column
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn rust_default_value(
        &self,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let Some(column_default) = &self.column_default else {
            return Err(WebCodeGenError::ColumnDoesNotHaveDefaultValue(Box::new(self.clone())));
        };
        let rust_str_data_type = self.str_rust_data_type(conn)?;
        let default = DefaultTypes::new(&rust_str_data_type, column_default)?;
        Ok(match (rust_str_data_type.as_str(), default) {
            (
                "::chrono::NaiveDateTime" | "chrono::NaiveDateTime",
                DefaultTypes::CurrentTimestamp,
            ) => {
                quote::quote! {
                    chrono::Local::now().naive_local()
                }
            }
            (
                "::rosetta_timestamp::TimestampUTC" | "rosetta_timestamp::TimestampUTC",
                DefaultTypes::CurrentTimestamp,
            ) => {
                quote::quote! {
                    rosetta_timestamp::TimestampUTC::default()
                }
            }
            ("i16", DefaultTypes::I16(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("i32", DefaultTypes::I32(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("i64", DefaultTypes::I64(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("f32", DefaultTypes::F32(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("f64", DefaultTypes::F64(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("bool", DefaultTypes::Bool(value)) => {
                quote::quote! {
                    #value
                }
            }
            ("String", DefaultTypes::String(value)) => {
                quote::quote! {
                    #value.to_owned()
                }
            }
            ("::rosetta_uuid::Uuid" | "rosetta_uuid::Uuid", DefaultTypes::Uuid(value)) => value,
            (r#type, default) => {
                unimplemented!(
                    "Default value `{default:?}` for column \"{}\".\"{}\" of type `{}` is not implemented!",
                    self.table_name,
                    self.column_name,
                    r#type
                )
            }
        })
    }

    /// Returns whether the column contains the update user and is defined by
    /// the SESSION user
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_updated_by(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.column_name == "updated_by"
            && self.foreign_keys(conn)?.iter().any(|key| {
                let Ok(foreign_columns) = key.foreign_columns(conn) else {
                    return false;
                };

                foreign_columns.len() == 1
                    && foreign_columns[0].table_name == "users"
                    && foreign_columns[0].column_name == "id"
            }))
    }

    /// Returns whether the column contains the creation user and is defined by
    /// the SESSION user
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_created_by(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.column_name == "created_by"
            && self.foreign_keys(conn)?.iter().any(|key| {
                let Ok(foreign_columns) = key.foreign_columns(conn) else {
                    return false;
                };

                foreign_columns.len() == 1
                    && foreign_columns[0].table_name == "users"
                    && foreign_columns[0].column_name == "id"
            }))
    }

    /// Returns whether the column contains the most concrete table variant in
    /// an extension hierarchy
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_most_concrete_table(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        Ok(self.column_name == "most_concrete_table"
            && self.data_type == "text"
            && !self.is_nullable()
            && !self.table(conn)?.is_extension(conn)?)
    }

    #[must_use]
    /// Returns whether the column is a timestamp which has to be updated at
    /// each update operation
    pub fn is_updated_at(&self) -> bool {
        self.column_name == "updated_at" && self.data_type == "timestamp with time zone"
    }

    #[must_use]
    /// Returns whether the column is a timestamp which has to be set at the
    /// insert operation
    pub fn is_created_at(&self) -> bool {
        self.column_name == "created_at" && self.data_type == "timestamp with time zone"
    }

    /// Load a column with a given name fom a given table
    ///
    /// # Arguments
    ///
    /// * `column_name` - The name of the column
    /// * `table_name` - The name of the table
    /// * `table_schema` - The schema of the table
    /// * `table_catalog` - The catalog of the table
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Column` if the operation was
    /// successful,
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn load(
        column_name: &str,
        table_name: &str,
        table_schema: &str,
        table_catalog: &str,
        conn: &mut PgConnection,
    ) -> Result<Option<Self>, WebCodeGenError> {
        use crate::schema::columns;

        Ok(columns::table
            .filter(
                columns::column_name.eq(column_name).and(
                    columns::table_name
                        .eq(table_name)
                        .and(columns::table_schema.eq(table_schema))
                        .and(columns::table_catalog.eq(table_catalog)),
                ),
            )
            .first(conn)
            .optional()?)
    }

    /// Returns the foreign table of the column if it is a foreign key.
    /// If the column is not a foreign key, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<KeyColumnUsage>>, WebCodeGenError> {
        foreign_keys(self, conn)
    }

    /// Returns whether the column has foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    pub fn has_foreign_keys(&self, conn: &mut PgConnection) -> bool {
        self.foreign_keys(conn).map_or(false, |keys| !keys.is_empty())
    }

    /// Returns whether the column is a foreign key with `ON DELETE CASCADE`
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether the column is a foreign key with `ON DELETE
    /// CASCADE` constraint
    pub fn is_foreign_key_on_delete_cascade(&self, conn: &mut PgConnection) -> bool {
        self.foreign_keys(conn).map_or(false, |keys| {
            keys.iter().any(|key| key.has_on_delete_cascade(conn).unwrap_or(false))
        })
    }

    /// Returns the getter method name for the column.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    ///
    /// # Returns
    ///
    /// A `Result` containing the getter method name if the operation was
    /// successful,
    pub fn getter_name(&self) -> Result<String, WebCodeGenError> {
        let mut snake_case_name = self.snake_case_name()?;
        if let Some(stripped_snake_case_name) = snake_case_name.strip_suffix("_id") {
            snake_case_name = stripped_snake_case_name.to_owned();
        }

        Ok(snake_case_name)
    }

    /// Returns the getter method ident for the column.
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column name
    ///
    /// # Returns
    ///
    /// A `Result` containing the getter method ident if the operation was
    /// successful,
    pub fn getter_ident(&self) -> Result<Ident, WebCodeGenError> {
        let getter_name = self.getter_name()?;
        if RESERVED_RUST_WORDS.contains(&getter_name.as_str()) {
            Ok(Ident::new_raw(&getter_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&getter_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns the multi-column getter method name for the provided columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A slice of `Column` references
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column names
    pub fn multi_column_getter_name<C: AsRef<Column>>(
        columns: &[C],
    ) -> Result<String, WebCodeGenError> {
        Ok(columns
            .iter()
            .map(|c| c.as_ref().getter_name())
            .collect::<Result<Vec<String>, WebCodeGenError>>()?
            .join("_and_"))
    }

    /// Returns the multi-column getter method ident for the provided columns.
    ///
    /// # Arguments
    ///
    /// * `columns` - A slice of `Column` references
    ///
    /// # Errors
    ///
    /// * If an error occurs while sanitizing the column names
    pub fn multi_column_getter_ident<C: AsRef<Column>>(
        columns: &[C],
    ) -> Result<Ident, WebCodeGenError> {
        let getter_name = Self::multi_column_getter_name(columns)?;
        if RESERVED_RUST_WORDS.contains(&getter_name.as_str()) {
            Ok(Ident::new_raw(&getter_name, proc_macro2::Span::call_site()))
        } else {
            Ok(Ident::new(&getter_name, proc_macro2::Span::call_site()))
        }
    }

    /// Returns whether the column is part of the table's primary key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_part_of_primary_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let table = self.table(conn)?;
        let primary_key_columns = table.primary_key_columns(conn)?;
        Ok(primary_key_columns.contains(self))
    }

    /// Returns whether the column coincides with the table primary key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_primary_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let table = self.table(conn)?;
        let primary_key_columns = table.primary_key_columns(conn)?;
        Ok(primary_key_columns.len() == 1 && primary_key_columns.contains(self))
    }

    /// Returns whether the column is part of an extension primary key
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_part_of_extension_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .find(|key| key.is_extension(conn).unwrap_or(false))
            .cloned())
    }

    /// Returns whether the column is to be handled as a partial builder.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    ///
    /// # Implementation details
    ///
    /// A column must be handled as a partial builder if:
    /// - The column C from table T is not nullable
    /// - The table T is an extension table, meaning it extends some other table
    ///   E.
    /// - The column C from table T is a foreign key to the primary key of the
    ///   table F, and F != E.
    /// - The table F has a same-as UNIQUE index constraint on the primary key
    ///   of the table E.
    /// - The table T has a foreign key same-as constraint to the same-as UNIQUE
    ///   index constraint of the table F.
    pub fn requires_partial_builder(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(PartialBuilderKind, KeyColumnUsage, KeyColumnUsage)>, WebCodeGenError> {
        let mut partial_builder_foreign_keys = Vec::new();

        for foreign_key in self.foreign_keys(conn)?.as_ref() {
            let Some((partial_builder_kind, potential_same_as_constraint)) =
                foreign_key.is_partial_builder_constraint(conn)?
            else {
                continue;
            };
            partial_builder_foreign_keys.push((
                partial_builder_kind,
                potential_same_as_constraint,
                foreign_key.clone(),
            ));
        }

        if partial_builder_foreign_keys.len() > 1 {
            unreachable!(
                "Column {self} seems to be is part of {} partial builder constraints, which is not supported. The builders include the columns: {}",
                partial_builder_foreign_keys.len(),
                partial_builder_foreign_keys
                    .iter()
                    .map(|(_, _, key)| {
                        key.columns(conn)
                            .unwrap_or_default()
                            .iter()
                            .map(|col| col.to_string())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .collect::<Vec<_>>()
                    .join(" | ")
            );
        }

        Ok(partial_builder_foreign_keys.pop())
    }

    /// Returns the foreign keys that are foreign primary keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn foreign_primary_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .filter(|key| key.is_foreign_primary_key(conn).unwrap_or(false))
            .cloned()
            .collect())
    }

    /// Returns whether the column is a foreign primary key of the provided
    /// table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_primary_key_of_table(
        &self,
        table: &Table,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
        for foreign_primary_key in self.foreign_primary_keys(conn)? {
            let foreign_table = foreign_primary_key.foreign_table(conn)?;
            if foreign_table.as_ref() == table || foreign_table.is_extending(table, conn)? {
                return Ok(Some(foreign_primary_key));
            }
        }
        Ok(None)
    }

    /// Returns whether the column is a foreign primary key.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_foreign_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(!self.foreign_primary_keys(conn)?.is_empty())
    }

    /// Returns the ancestral same-as constraints for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn ancestral_same_as_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        Ok(self
            .foreign_keys(conn)?
            .iter()
            .filter(|foreign_key| {
                foreign_key
                    .is_ancestral_same_as_constraint(conn)
                    .map(|index| index.is_some())
                    .unwrap_or(false)
            })
            .cloned()
            .collect())
    }

    /// Returns the associated same-as constraints for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `include_local_primary_key` - Whether to include the local primary key
    ///   in the constraint
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn associated_same_as_constraints(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        let mut associated_same_as_constraints = Vec::new();

        for foreign_key in self.foreign_keys(conn)?.iter() {
            if foreign_key
                .is_associated_same_as_constraint(include_local_primary_key, conn)?
                .is_none()
            {
                continue;
            }
            associated_same_as_constraints.push(foreign_key.clone());
        }

        Ok(associated_same_as_constraints)
    }

    /// Returns whether the current column is a foreign definer.
    ///
    /// # Arguments
    ///
    /// * `conn`: A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_foreign_definer(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(!self.foreign_definer_constraints(conn)?.is_empty())
    }

    pub(crate) fn foreign_definer_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, WebCodeGenError> {
        if let Some((PartialBuilderKind::Mandatory, _, _)) = self.requires_partial_builder(conn)? {
            return Ok(Vec::new());
        }

        let mut foreign_definer_constraints = Vec::new();
        for foreign_key in self.foreign_keys(conn)?.iter() {
            let foreign_table = foreign_key.foreign_table(conn)?;
            if !self.is_foreign_primary_key_of_table(&foreign_table, conn)?.is_some() {
                continue;
            }
            if foreign_key.includes_local_primary_key(conn)? {
                continue;
            }
            if !foreign_key.includes_foreign_primary_key(conn)? {
                continue;
            }
            if foreign_table.primary_key_columns(conn)?.len()
                == foreign_key.foreign_columns(conn)?.len()
            {
                continue;
            }
            if foreign_key.is_same_as_constraint(conn)?.is_some() {
                foreign_definer_constraints.push(foreign_key.clone());
            }
        }

        Ok(foreign_definer_constraints)
    }

    pub(crate) fn foreign_definer_constraints_by_table(
        &self,
        conn: &mut PgConnection,
    ) -> Result<HashMap<Arc<Table>, Vec<KeyColumnUsage>>, WebCodeGenError> {
        let mut foreign_definer_constraints_by_table = HashMap::new();
        for foreign_definer_constraint in self.foreign_definer_constraints(conn)? {
            let foreign_table = foreign_definer_constraint.foreign_table(conn)?;
            foreign_definer_constraints_by_table
                .entry(foreign_table)
                .or_insert_with(Vec::new)
                .push(foreign_definer_constraint);
        }
        Ok(foreign_definer_constraints_by_table)
    }

    /// Returns the set of columns which are uniquely defined by values
    /// associated with the foreign table associated with the column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn foreign_defined_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut foreign_defined_columns = Vec::new();
        for foreign_definer_constraint in self.foreign_definer_constraints(conn)? {
            foreign_defined_columns.extend(
                foreign_definer_constraint.columns(conn)?.iter().filter(|c| c != &self).cloned(),
            );
        }

        foreign_defined_columns.sort_unstable();

        Ok(foreign_defined_columns)
    }

    /// Returns the set of columns which uniquely define the current column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn foreign_definer_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        let mut foreign_definer_columns = Vec::new();
        let table = self.table(conn)?;
        for column in table.columns(conn)?.iter() {
            if column == self {
                continue;
            }
            if column.foreign_defined_columns(conn)?.contains(self) {
                foreign_definer_columns.push(column.clone());
            }
        }
        Ok(foreign_definer_columns)
    }

    /// Returns the ancestral same-as columns for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn ancestral_same_as_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Column>, WebCodeGenError> {
        Ok(self
            .ancestral_same_as_constraints(conn)?
            .into_iter()
            .map(|constraint| {
                let local_columns = constraint.columns(conn)?;
                let foreign_columns = constraint.foreign_columns(conn)?;
                Ok(local_columns
                    .iter()
                    .zip(foreign_columns.iter())
                    .filter_map(|(local_column, foreign_column)| {
                        if local_column == self { Some(foreign_column.clone()) } else { None }
                    })
                    .next()
                    .unwrap())
            })
            .collect::<Result<Vec<Column>, WebCodeGenError>>()?)
    }

    /// Returns the ancestral same-as reachable set for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn ancestral_same_as_reachable_set(
        &self,
        conn: &mut PgConnection,
    ) -> Result<HashSet<Column>, WebCodeGenError> {
        ancestral_same_as_reachable_set(self, conn)
    }

    /// Returns the associated same-as columns for the column, if any.
    ///
    /// # Arguments
    ///
    /// * `include_local_primary_key` - Whether to include the local primary key
    ///   in the constraint
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn associated_same_as_columns(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, KeyColumnUsage)>, WebCodeGenError> {
        associated_same_as_columns(self, include_local_primary_key, conn)
    }

    /// Returns the normal and inferred ancestral same-as constraints for the
    /// column, if any.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn all_ancestral_same_as_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Self>, WebCodeGenError> {
        all_ancestral_same_as_columns(self, conn)
    }

    /// Returns the set of unique tables that are referenced by foreign keys
    /// associated to the current column where the current column is a primary
    /// key in the foreign table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn referenced_by_foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<Arc<Table>>, WebCodeGenError> {
        let mut referenced_tables = Vec::new();

        if self.is_primary_key(conn)? {
            let table = self.table(conn)?;
            referenced_tables.push(table);
        }

        for key in self.foreign_keys(conn)?.iter() {
            if key.is_foreign_primary_key(conn)? {
                let foreign_table = key.foreign_table(conn)?;
                referenced_tables.push(foreign_table);
            }
        }

        referenced_tables.sort_unstable();
        referenced_tables.dedup();

        Ok(referenced_tables)
    }

    /// Returns whether the current column is ancestrally same-as the provided
    /// column.
    ///
    /// # Implementative details
    ///
    /// A column is ancestrally same-as another column if:
    ///
    /// - The two columns are linked by a same-as constraint or inferred to be.
    /// - The same-as constraint includes the primary key of the table of the
    ///   current column.
    ///
    /// # Arguments
    ///
    /// * `other` - The other column to compare with
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_ancestrally_same_as(
        &self,
        other: &Column,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        self.all_ancestral_same_as_columns(conn).map(|columns| columns.contains(other))
    }

    /// Returns the distinct check constraints that apply to the column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, WebCodeGenError> {
        let check_constraints = self.check_constraints(conn)?;
        let mut distinct_check_constraints = Vec::new();

        for check_constraint in check_constraints.iter() {
            if check_constraint.is_distinct_constraint(conn)?.is_some() {
                let involved_columns = check_constraint.columns(conn)?;
                let Some(distinct_column) = involved_columns.iter().find(|c| c != &self) else {
                    continue;
                };
                distinct_check_constraints
                    .push((distinct_column.clone(), check_constraint.clone()));
            }
        }

        Ok(distinct_check_constraints)
    }

    /// Returns the inherited distinct check constraints that apply to the
    /// column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn inherited_distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, WebCodeGenError> {
        let mut inherited_distinct_check_constraints = HashSet::new();
        let mut distinct_columns_and_constraints = Vec::new();
        let table_columns = self.table(conn)?.columns(conn)?;
        let mut possible_matching_columns = Vec::new();

        for column in table_columns.iter() {
            if column == self {
                continue;
            }
            if self.has_compatible_data_type(column, conn)? {
                possible_matching_columns.push(column);
            }
        }

        for column in self.all_ancestral_same_as_columns(conn)? {
            for (distinct_column, distinct_check_constraint) in
                column.distinct_check_constraints(conn)?
            {
                // If we have already added this constraint, skip it
                if inherited_distinct_check_constraints.contains(&distinct_check_constraint) {
                    continue;
                }

                for possible_matching_column in possible_matching_columns.iter() {
                    if possible_matching_column.is_ancestrally_same_as(&distinct_column, conn)? {
                        distinct_columns_and_constraints.push((
                            (*possible_matching_column).clone(),
                            distinct_check_constraint.clone(),
                        ));
                    }
                }

                inherited_distinct_check_constraints.insert(distinct_check_constraint);
            }
        }

        Ok(distinct_columns_and_constraints)
    }

    /// Returns all distinct check constraints that apply to the column,
    /// including inherited ones.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn all_distinct_check_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, CheckConstraint)>, WebCodeGenError> {
        let mut all_distinct_check_constraints = self.distinct_check_constraints(conn)?;
        all_distinct_check_constraints.extend(self.inherited_distinct_check_constraints(conn)?);
        Ok(all_distinct_check_constraints)
    }

    /// Returns whether the column has exactly one foreign key constraint
    /// that references exactly one column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn has_singleton_foreign_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.foreign_keys(conn)?.iter().any(|key| key.is_singleton(conn).unwrap_or(false)))
    }
}
