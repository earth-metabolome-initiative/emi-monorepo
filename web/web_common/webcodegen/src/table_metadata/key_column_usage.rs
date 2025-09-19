use std::{fmt::Display, sync::Arc};

use cached::{DiskCache, proc_macro::io_cached};
use diesel::{
    ExpressionMethods, PgConnection, QueryDsl, Queryable, QueryableByName, RunQueryDsl, Selectable,
    result::Error as DieselError,
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::{Column, Table};
use crate::{PgIndex, ReferentialConstraint, errors::WebCodeGenError, traits::TableLike};

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.referential_constraint")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{key_column_usage.clone()}"#
)]
fn referential_constraint(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<ReferentialConstraint, WebCodeGenError> {
    use diesel::SelectableHelper;

    use crate::schema::referential_constraints;
    Ok(referential_constraints::table
        .filter(referential_constraints::constraint_name.eq(&key_column_usage.constraint_name))
        .filter(referential_constraints::constraint_schema.eq(&key_column_usage.constraint_schema))
        .filter(
            referential_constraints::constraint_catalog.eq(&key_column_usage.constraint_catalog),
        )
        .select(ReferentialConstraint::as_select())
        .first::<ReferentialConstraint>(conn)?)
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.foreign_columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{key_column_usage.clone()}"#
)]
fn foreign_columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, SelectableHelper};

    use crate::schema::{columns, constraint_column_usage};

    // Find the referential constraint for this key_column_usage
    let referential_constraint = key_column_usage.referential_constraint(conn)?;

    // Find the columns in the referenced (unique) constraint
    Ok(Arc::new(
        constraint_column_usage::table
            .filter(constraint_column_usage::constraint_catalog.eq(
                referential_constraint.unique_constraint_catalog.ok_or(DieselError::NotFound)?,
            ))
            .filter(
                constraint_column_usage::constraint_schema.eq(referential_constraint
                    .unique_constraint_schema
                    .ok_or(DieselError::NotFound)?),
            )
            .filter(
                constraint_column_usage::constraint_name.eq(referential_constraint
                    .unique_constraint_name
                    .ok_or(DieselError::NotFound)?),
            )
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                    .and(columns::table_catalog.eq(constraint_column_usage::table_catalog))
                    .and(columns::column_name.eq(constraint_column_usage::column_name))),
            )
            .order_by(columns::ordinal_position.asc())
            .select(Column::as_select())
            .load::<Column>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.load_key_table")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{key_column_usage.clone()}"#
)]
fn load_key_table(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Arc<Table>, WebCodeGenError> {
    use diesel::SelectableHelper;

    use crate::schema::tables;

    Ok(Arc::new(
        tables::table
            .filter(tables::table_name.eq(&key_column_usage.table_name))
            .filter(tables::table_schema.eq(&key_column_usage.table_schema))
            .filter(tables::table_catalog.eq(&key_column_usage.table_catalog))
            .select(Table::as_select())
            .first::<Table>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.columns")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{key_column_usage.clone()}"#
)]
fn columns(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, SelectableHelper};

    use crate::schema::{columns, key_column_usage};
    Ok(Arc::new(
        key_column_usage::table
            .filter(key_column_usage::constraint_name.eq(&key_column_usage.constraint_name))
            .filter(key_column_usage::constraint_schema.eq(&key_column_usage.constraint_schema))
            .filter(key_column_usage::constraint_catalog.eq(&key_column_usage.constraint_catalog))
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(key_column_usage::table_name)
                    .and(columns::table_schema.eq(key_column_usage::table_schema))
                    .and(columns::table_catalog.eq(key_column_usage::table_catalog))
                    .and(columns::column_name.eq(key_column_usage::column_name))),
            )
            .order_by(key_column_usage::ordinal_position.asc())
            .select(Column::as_select())
            .load::<Column>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.foreign_table")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{key_column_usage.clone()}"#
)]
/// Returns the foreign table associated with this key column usage
///
/// # Arguments
///
/// * `conn` - A mutable reference to a `PgConnection`
///
/// # Errors
///
/// * If an error occurs while loading the foreign table from the database
fn foreign_table(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<Arc<Table>, WebCodeGenError> {
    use diesel::{BoolExpressionMethods, JoinOnDsl, SelectableHelper};

    use crate::schema::{constraint_table_usage, tables};

    let constraint = key_column_usage.referential_constraint(conn)?;

    Ok(Arc::new(
        constraint_table_usage::table
            .inner_join(
                tables::table.on(tables::table_name
                    .eq(constraint_table_usage::table_name)
                    .and(tables::table_schema.eq(constraint_table_usage::table_schema))
                    .and(tables::table_catalog.eq(constraint_table_usage::table_catalog))),
            )
            .filter(constraint_table_usage::constraint_name.eq(&constraint.constraint_name))
            .filter(constraint_table_usage::constraint_schema.eq(&constraint.constraint_schema))
            .filter(constraint_table_usage::constraint_catalog.eq(&constraint.constraint_catalog))
            .select(Table::as_select())
            .first::<Table>(conn)?,
    ))
}

#[io_cached(
    map_error = r##"|e| WebCodeGenError::from(e)"##,
    disk = true,
    sync_to_disk_on_cache_change = true,
    create = r##" {
        DiskCache::new("key_column_usage.is_extension")
            .set_disk_directory("cache")
            .build()
            .expect("error building disk cache")
    } "##,
    key = "KeyColumnUsage",
    convert = r#"{ key_column_usage.clone() }"#
)]
pub fn is_extension(
    key_column_usage: &KeyColumnUsage,
    conn: &mut PgConnection,
) -> Result<bool, WebCodeGenError> {
    Ok(key_column_usage.is_foreign_primary_key(conn)?
        && key_column_usage.is_local_primary_key(conn)?
        && !key_column_usage.is_self_referential(conn)?)
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
/// The kind of partial builder constraint
pub enum PartialBuilderKind {
    /// The partial builder constraint is discretionary, i.e. the user
    /// may provide either the primary key or the builder of the associated
    /// table when creating a new instance of the host table.
    Discretional,
    /// The partial builder constraint is mandatory, i.e. the user must
    /// use a partial builder of the associated table when creating a new
    /// instance of the host table.
    Mandatory,
}

impl PartialBuilderKind {
    /// Returns whether the partial builder constraint is discretionary
    pub fn is_discretional(self) -> bool {
        matches!(self, PartialBuilderKind::Discretional)
    }

    /// Returns the formatted type of the partial builder constraint.
    ///
    /// # Arguments
    ///
    /// * `table` - The table associated with the partial builder constraint.
    /// * `conn` - A mutable reference to a `PgConnection`.
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database.
    pub(crate) fn formatted_type<T: AsRef<Table>>(
        self,
        table: &T,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        let builder_type = table.as_ref().insertable_builder_ty()?;
        match self {
            PartialBuilderKind::Discretional => {
                let primary_key_type = table.as_ref().primary_key_type(conn)?;
                Ok(
                    quote! { web_common_traits::database::IdOrBuilder<#primary_key_type, #builder_type> },
                )
            }
            PartialBuilderKind::Mandatory => Ok(quote! { #builder_type }),
        }
    }
}

/// Represents a row in the `key_column_usage` table, which contains information
/// about columns that are constrained by a unique or primary key constraint.
///
/// For more details, see [`PostgreSQL`](https://www.postgresql.org/docs/current/infoschema-key-column-usage.html)
#[derive(
    Queryable,
    QueryableByName,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Ord,
    PartialOrd,
    Clone,
    Hash,
    serde::Serialize,
    serde::Deserialize,
)]
#[diesel(table_name = crate::schema::key_column_usage)]
pub struct KeyColumnUsage {
    /// The name of the database that contains the constraint.
    pub constraint_catalog: String,
    /// The name of the schema that contains the constraint.
    pub constraint_schema: String,
    /// The name of the constraint.
    pub constraint_name: String,
    /// The name of the database that contains the table.
    pub table_catalog: String,
    /// The name of the schema that contains the table.
    pub table_schema: String,
    /// The name of the table that contains the column.
    pub table_name: String,
    /// The name of the column that is constrained.
    pub column_name: String,
    /// The position of the column within the constraint.
    pub ordinal_position: i32,
    /// The position of the column within the unique constraint, if applicable.
    pub position_in_unique_constraint: Option<i32>,
}

impl Display for KeyColumnUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}.{}({})",
            self.table_schema, self.table_name, self.column_name, self.constraint_name
        )
    }
}

impl AsRef<KeyColumnUsage> for KeyColumnUsage {
    fn as_ref(&self) -> &KeyColumnUsage {
        self
    }
}

impl KeyColumnUsage {
    /// Returns the SQL definition of the key column usage as a foreign key
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn to_sql(&self, conn: &mut PgConnection) -> Result<String, WebCodeGenError> {
        let local_columns = self.columns(conn)?;
        let foreign_table = self.foreign_table(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        let local_column_names: Vec<String> =
            local_columns.iter().map(|col| col.column_name.clone()).collect();
        let foreign_column_names: Vec<String> =
            foreign_columns.iter().map(|col| col.column_name.clone()).collect();

        let on_delete_clause =
            if self.has_on_delete_cascade(conn)? { " ON DELETE CASCADE" } else { "" };

        Ok(format!(
            "FOREIGN KEY ({}) REFERENCES {}.{}({}){}",
            local_column_names.join(", "),
            foreign_table.table_schema,
            foreign_table.table_name,
            foreign_column_names.join(", "),
            on_delete_clause
        ))
    }

    /// Returns whether the key is a singleton foreign key, i.e. it is the only
    /// foreign key to refer to a particular foreign table within the context
    /// of its table of definition.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_singleton(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        if self.is_local_primary_key(conn)? {
            return Ok(false);
        }
        if self.columns(conn)?.len() != 1 {
            return Ok(false);
        }
        let foreign_table = self.foreign_table(conn)?;
        let table = self.table(conn)?;
        Ok(table.foreign_keys(conn)?.iter().all(|fk| {
            fk == self || fk.foreign_table(conn).map(|t| t != foreign_table).unwrap_or(true)
        }))
    }

    /// Returns whether the key is on delete cascade
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn has_on_delete_cascade(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let referential_constraint = self.referential_constraint(conn)?;
        Ok(referential_constraint.delete_rule == "CASCADE")
    }

    /// Returns the referential constraint associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the referential constraint from the
    ///   database
    pub(crate) fn referential_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<ReferentialConstraint, WebCodeGenError> {
        referential_constraint(self, conn)
    }

    /// Returns the table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the table from the database
    pub(crate) fn table(&self, conn: &mut PgConnection) -> Result<Arc<Table>, WebCodeGenError> {
        load_key_table(self, conn)
    }

    /// Returns whether the key column usage is self-referential, i.e. the
    /// foreign table is the same as the local table
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    pub(crate) fn is_self_referential(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.table(conn)? == self.foreign_table(conn)?)
    }

    /// Returns the foreign table associated with this key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign table from the database
    pub fn foreign_table(&self, conn: &mut PgConnection) -> Result<Arc<Table>, WebCodeGenError> {
        foreign_table(self, conn)
    }

    /// Returns all the columns involved in the constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub fn columns(&self, conn: &mut PgConnection) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
        columns(self, conn)
    }

    /// Returns the column mapping between the local and foreign columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the column mappings from the database
    pub fn column_mappings(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(Column, Column)>, WebCodeGenError> {
        Ok(self
            .columns(conn)?
            .as_ref()
            .iter()
            .cloned()
            .zip(self.foreign_columns(conn)?.as_ref().iter().cloned())
            .collect())
    }

    /// Returns whether it is a composite key column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub(crate) fn is_composite(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.columns(conn).map(|columns| columns.len() > 1)
    }

    /// Returns whether any column involved in the constraint is nullable
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the key column usages from the
    ///   database
    pub(crate) fn is_nullable(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        self.columns(conn).map(|columns| columns.iter().any(Column::is_nullable))
    }

    /// Returns the columns in the foreign table that are referenced by this key
    /// column usage
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while loading the foreign key column usages from
    ///   the database
    pub fn foreign_columns(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Arc<Vec<Column>>, WebCodeGenError> {
        foreign_columns(self, conn)
    }

    /// Returns whether the key column usage refers to a foreign primary key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_primary_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        let foreign_table = self.foreign_table(conn)?;

        // Check if the foreign table has a primary key
        let primary_keys = foreign_table.primary_key_columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        Ok(primary_keys == foreign_columns)
    }

    /// Returns whether the key column usage refers to a local primary key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_local_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let table = self.table(conn)?;

        // Check if the table has a primary key
        let primary_keys = table.primary_key_columns(conn)?;
        let columns = self.columns(conn)?;
        Ok(primary_keys == columns)
    }

    /// Returns whether the key column usage includes the local primary key
    /// columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn includes_local_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let table = self.table(conn)?;
        let primary_keys = table.primary_key_columns(conn)?;
        let columns = self.columns(conn)?;
        Ok(primary_keys.iter().all(|pk| columns.contains(pk)))
    }

    /// Returns whether the key column usage includes the foreign primary key
    /// columns
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn includes_foreign_primary_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        let foreign_table = self.foreign_table(conn)?;
        let primary_keys = foreign_table.primary_key_columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        Ok(primary_keys.iter().all(|pk| foreign_columns.contains(pk)))
    }

    /// Returns whether the key column usage refers to a foreign unique key
    /// constraint
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_unique_key(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        let foreign_table = self.foreign_table(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        // Check if the foreign table has a unique key constraint
        for unique_constraint in foreign_table.unique_indices(conn)? {
            // Check if the foreign columns match the unique constraint columns
            let unique_columns = unique_constraint.columns(conn)?;
            if unique_columns.len() == foreign_columns.len()
                && unique_columns.iter().all(|c| foreign_columns.contains(c))
            {
                return Ok(Some(unique_constraint));
            }
        }
        Ok(None)
    }

    /// Returns whether this key column usage defines an extension.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_extension(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        is_extension(self, conn)
    }

    /// Returns the identifier for this key column usage getter.
    ///
    /// # Implementation details
    ///
    /// The name of the constraint is defined as follows:
    ///
    /// * If the constraint refers to several columns, the name is the name of
    ///   the constraint as defined in the database.
    /// * If the constraint refers to a single column, but there exist some
    ///   other single-column constraint which also refers to the same column,
    ///   the name is the name of the constraint as defined in the database.
    /// * Otherwise, the name is the getter identifier of the column.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn constraint_ident(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Ident, WebCodeGenError> {
        let columns = self.columns(conn)?;
        if columns.len() == 1 {
            // We check whether there exist some other constraint which also refers
            // to the same column in a single-column constraint.
            let column = &columns[0];
            let constraints = column.foreign_keys(conn)?;
            let mut has_other_constraints = false;
            for constraint in constraints.as_ref() {
                if constraint == self {
                    continue;
                }
                if constraint.columns(conn)?.len() == 1 {
                    has_other_constraints = true;
                    break;
                }
            }
            if !has_other_constraints {
                // If there are no other constraints, we use the getter identifier
                return column.getter_ident();
            }
        }

        let mut snake_case_name = self.constraint_name.to_lowercase();
        while snake_case_name.contains("__") {
            snake_case_name = snake_case_name.replace("__", "_");
        }

        Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
    }

    /// Returns the where statement for this key column usage
    ///
    /// # Arguments
    ///
    /// * `include_self` - A boolean indicating whether to include the `self`
    ///   reference in the where statement.
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn where_statement(
        &self,
        foreign_table: bool,
        include_self: bool,
        conn: &mut PgConnection,
    ) -> Result<TokenStream, WebCodeGenError> {
        // Analogously, we check before executing the query whether the current column
        // is None. If so, we return None as well.
        let mut where_statement = TokenStream::new();

        let columns = self.columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;
        let foreign_key_table = self.foreign_table(conn)?;
        let current_table = self.table(conn)?;
        let foreign_table_path = foreign_key_table.import_diesel_path()?;
        let current_table_path = current_table.import_diesel_path()?;

        assert!(!columns.is_empty(), "The key column usage must have at least one column {self:?}",);

        assert!(
            !foreign_columns.is_empty(),
            "The foreign key must have at least one column {self:?}",
        );

        assert_eq!(
            columns.len(),
            foreign_columns.len(),
            "The number of columns in the key column usage must match the number of foreign columns",
        );

        assert!(
            foreign_columns.iter().all(|c| c.table_name == foreign_key_table.table_name),
            "Error while processing table `{}.{}`'s FK `{}`: All foreign columns must belong to the same table `{}` as the foreign key, but got {foreign_columns:?}",
            self.table_schema,
            self.table_name,
            self.constraint_name,
            foreign_key_table.table_name
        );

        for (column, foreign_column) in columns.iter().zip(foreign_columns.iter()) {
            let current_column_ident: Ident = column.snake_case_ident()?;
            let foreign_column_ident: Ident = foreign_column.snake_case_ident()?;

            let column_attribute = if column.is_nullable() || !include_self {
                quote::quote! { #current_column_ident }
            } else {
                quote::quote! { &self.#current_column_ident }
            };

            let single_where_statement = if foreign_table {
                quote::quote! {
                    #foreign_table_path::dsl::#foreign_column_ident.eq(#column_attribute)
                }
            } else {
                quote::quote! {
                    #current_table_path::dsl::#current_column_ident.eq(#column_attribute)
                }
            };

            if where_statement.is_empty() {
                where_statement = single_where_statement;
            } else {
                where_statement = quote::quote! {
                    #where_statement.and(#single_where_statement)
                };
            }
        }

        Ok(where_statement)
    }

    /// Returns whether this key column usage is a `same-as` constraint
    ///
    /// A `same-as` constraint is a composite foreign key that refers to a
    /// UNIQUE constraint, where the foreign key's foreign columns are the same
    /// as the primary key of the foreign table, and the foreign column
    /// corresponding to the current column is part of the primary key of the
    /// foreign table.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_same_as_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        let Some(foreign_unique_constraint) = self.is_foreign_unique_key(conn)? else {
            return Ok(None);
        };

        Ok(if foreign_unique_constraint.is_same_as(conn)? {
            // If the foreign unique constraint is a same-as constraint, we return it
            Some(foreign_unique_constraint)
        } else {
            // Otherwise, we return None
            None
        })
    }

    /// Returns whether this key column usage is an ancestral same-as
    /// constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_ancestral_same_as_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        if !self.includes_local_primary_key(conn)? {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;
        let table = self.table(conn)?;

        if !table.is_extending(&foreign_table, conn)? {
            return Ok(None);
        }

        self.is_same_as_constraint(conn)
    }

    /// Returns whether this key column usage is an associated same-as
    /// constraint.
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
    pub fn is_associated_same_as_constraint(
        &self,
        include_local_primary_key: bool,
        conn: &mut PgConnection,
    ) -> Result<Option<PgIndex>, WebCodeGenError> {
        if !include_local_primary_key && self.includes_local_primary_key(conn)? {
            return Ok(None);
        }
        if !self
            .columns(conn)?
            .iter()
            .any(|c| c.requires_partial_builder(conn).ok().flatten().is_some())
        {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;

        let table = self.table(conn)?;

        if table.is_extending(&foreign_table, conn)? {
            return Ok(None);
        }

        self.is_same_as_constraint(conn)
    }

    /// Returns whether this constraint may be either a partial builder
    /// of a potential partial builder constraint.
    fn preliminary_partial_builder_check(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<Vec<KeyColumnUsage>>, WebCodeGenError> {
        // A partial builder constraint must be a foreign primary key.
        // and if the foreign key is self-referential, we do not consider it
        // a partial builder constraint.
        if self.is_self_referential(conn)? || !self.is_foreign_primary_key(conn)? {
            return Ok(None);
        }

        let foreign_table = self.foreign_table(conn)?;
        let host_table = self.table(conn)?;
        // If the source table is an extension of the foreign table,
        // we do not consider it a partial builder constraint.
        if host_table.is_extending(&foreign_table, conn)?
            || host_table.share_ancestors(&foreign_table, conn)?
        {
            return Ok(None);
        }

        // At this point, we need to identify foreign keys in the
        // foreign table which point to ancestors of the current table.
        let mut keys_to_local_ancestors = Vec::new();
        let primary_key_columns = host_table.primary_key_columns(conn)?;
        for foreign_key in foreign_table.foreign_keys(conn)?.as_ref() {
            let foreign_columns = foreign_key.foreign_columns(conn)?;

            if foreign_columns.len() != primary_key_columns.len() {
                continue;
            }

            if foreign_columns.iter().zip(primary_key_columns.iter()).all(
                |(foreign_column, primary_key_column)| {
                    primary_key_column.is_ancestrally_same_as(foreign_column, conn).unwrap_or(false)
                },
            ) {
                keys_to_local_ancestors.push(foreign_key.clone());
            }
        }

        if keys_to_local_ancestors.is_empty() {
            return Ok(None);
        }

        // While it is conceivable to define partial builders on the foreign keys
        // and not on the columns themselves, at this time we are proceeding solely
        // with a column-based approach. Hence, we only support single-column foreign
        // keys.
        if foreign_table.has_composite_primary_key(conn)? {
            unimplemented!(
                "Partial builders from table {host_table} to table {foreign_table} on composite foreign keys are not supported yet"
            );
        }

        Ok(Some(keys_to_local_ancestors))
    }

    /// Returns whether this key column usage is a potential partial builder
    /// same-as constraint.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Implementation details
    ///
    /// A potential partial builder constraint is a constraint that
    /// differs from a partial builder constraint in that the host
    /// table does not have the same-as constraint which closes the triangular
    /// relationship, and therefore the associated table may require the primary
    /// key of an ancestor of the host table to be built, but it also may not.
    /// Such distintion is modelled by requiring these structs to have as type
    /// parameter `IdOrBuilder<PK, B>`, where `PK` is the primary key type of
    /// the associated table, and `B` is the builder type of the associated
    /// table. This way, the user of the API can choose to provide either
    /// the primary key or the builder type when creating a new instance of
    /// the associated table.
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub(crate) fn is_partial_builder_constraint(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(PartialBuilderKind, KeyColumnUsage)>, WebCodeGenError> {
        // First, we perform some preliminary checks to rule out
        // constraints that cannot possibly be partial builders.
        let Some(keys_to_local_ancestors) = self.preliminary_partial_builder_check(conn)? else {
            return Ok(None);
        };

        // As described in the method documentation, what distinguishes
        // a potential partial builder constraint from a partial builder
        // constraint is the absence of same-as constraints in the host table
        // pointing to foreign keys in the foreign table which point to
        // ancestors of the host table.

        // We determine the local columns of the constraints pointing to
        // ancestors of the host table.
        let columns_to_local_ancestors = keys_to_local_ancestors
            .iter()
            .map(|key| key.columns(conn))
            .collect::<Result<Vec<Arc<Vec<Column>>>, WebCodeGenError>>()?;

        let table = self.table(conn)?;
        let local_columns = self.columns(conn)?;
        let foreign_columns = self.foreign_columns(conn)?;

        // For each foreign key in the host table, we check whether it contains
        // references to the specific ID contained in the local & foreign columns
        // of the current constraint, which implicitly also checks whether the
        // foreign key points to the same foreign table as the current constraint.
        // Next, we check whether the foreign key's foreign columns contain
        // any of the columns pointing to ancestors of the host table described
        // in constraints to ancestors of the host table which we determined above.
        for foreign_key in table.foreign_keys(conn)?.as_ref() {
            // We retrieve the local columns of the foreign key we are checking.
            let fk_local_columns = foreign_key.columns(conn)?;
            // If all of the columns involved in the current constraint are
            // present in the local columns of the foreign key, we proceed
            // to check the foreign columns.
            if !local_columns.iter().all(|c| fk_local_columns.contains(c)) {
                continue;
            }

            let fk_foreign_columns = foreign_key.foreign_columns(conn)?;
            // All of the foreign columns of the current constraint must
            // be present in the foreign columns of the foreign key.
            if !foreign_columns.iter().all(|c| fk_foreign_columns.contains(c)) {
                continue;
            }

            // Now that we have established that the foreign key involves
            // all of the local & foreign columns of the current constraint,
            // we need to find at least one case where the foreign key's
            // foreign columns contain all of the columns in a `columns_to_local_ancestors`.
            for columns_to_local_ancestor in &columns_to_local_ancestors {
                if columns_to_local_ancestor.iter().all(|c| fk_foreign_columns.contains(c)) {
                    return Ok(Some((PartialBuilderKind::Mandatory, foreign_key.clone())));
                }
            }
        }

        // In the case of a discretional partial builder constraint, we also need
        // to check that there exist only one `keys_to_local_ancestors`, otherwise
        // it would not be clear which of these columns to set.
        assert_eq!(
            keys_to_local_ancestors.len(),
            1,
            "Discretional partial builder constraints must have exactly one key to a local ancestor."
        );

        Ok(Some((PartialBuilderKind::Discretional, keys_to_local_ancestors[0].clone())))
    }
}
