use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, OptionalExtension, PgConnection, QueryDsl,
    Queryable, QueryableByName, RunQueryDsl, Selectable, SelectableHelper,
    result::Error as DieselError,
};
use proc_macro2::TokenStream;
use syn::{Ident, Type};

mod default_types;
pub use default_types::DefaultTypes;

use super::{
    check_constraint::CheckConstraint,
    pg_type::{COPY_TYPES, EQ_TYPES, HASH_TYPES, ORD_TYPES, PgType, rust_type_str},
    table::{RESERVED_DIESEL_WORDS, RESERVED_RUST_WORDS},
};
use crate::{
    KeyColumnUsage, Table, errors::WebCodeGenError,
    table_metadata::pg_type::postgres_type_to_diesel,
};

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Selectable, PartialEq, Eq, Debug, Clone, Hash)]
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
        self.geometry(conn).is_ok()
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
    ) -> Result<Vec<CheckConstraint>, WebCodeGenError> {
        use crate::schema::{check_constraints, constraint_column_usage};
        Ok(check_constraints::table
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
                constraint_column_usage::column_name.eq(&self.column_name).and(
                    constraint_column_usage::table_catalog.eq(&self.table_catalog).and(
                        constraint_column_usage::table_schema
                            .eq(&self.table_schema)
                            .and(constraint_column_usage::table_name.eq(&self.table_name)),
                    ),
                ),
            )
            .select(CheckConstraint::as_select())
            .load(conn)?)
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
    ) -> Result<crate::GeometryColumn, WebCodeGenError> {
        use crate::schema::geometry_columns;

        Ok(geometry_columns::table
            .filter(geometry_columns::f_table_name.eq(&self.table_name))
            .filter(geometry_columns::f_table_schema.eq(&self.table_schema))
            .filter(geometry_columns::f_geometry_column.eq(&self.column_name))
            .first::<crate::GeometryColumn>(conn)?)
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
    ) -> Result<crate::GeographyColumn, WebCodeGenError> {
        use crate::schema::geography_columns;

        Ok(geography_columns::table
            .filter(geography_columns::f_table_name.eq(&self.table_name))
            .filter(geography_columns::f_table_schema.eq(&self.table_schema))
            .filter(geography_columns::f_geography_column.eq(&self.column_name))
            .first::<crate::GeographyColumn>(conn)?)
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
    pub fn pg_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
        use crate::schema::{pg_attribute, pg_class, pg_namespace, pg_type};

        pg_type::table
            .inner_join(pg_attribute::table.on(pg_attribute::atttypid.eq(pg_type::oid)))
            .inner_join(pg_class::table.on(pg_attribute::attrelid.eq(pg_class::oid)))
            .inner_join(pg_namespace::table.on(pg_class::relnamespace.eq(pg_namespace::oid)))
            .filter(pg_class::relname.eq(&self.table_name))
            .filter(pg_namespace::nspname.eq(&self.table_schema))
            .filter(pg_attribute::attname.eq(&self.column_name))
            .select(PgType::as_select())
            .first::<PgType>(conn)
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
        if let Ok(geometry) = self.geometry(conn) {
            return Ok(geometry.str_rust_type().to_owned());
        }
        if let Ok(geography) = self.geography(conn) {
            return Ok(geography.str_rust_type().to_owned());
        }
        match rust_type_str(self.data_type_str(conn)?, conn) {
            Ok(s) => Ok(s.to_string()),
            Err(error) => {
                if self.has_custom_type() {
                    Ok(PgType::from_name(self.data_type_str(conn)?, conn)?.camelcased_name())
                } else {
                    Err(error)
                }
            }
        }
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
    pub fn has_compatible_data_type(
        &self,
        column: &Column,
        conn: &mut PgConnection,
    ) -> Result<bool, WebCodeGenError> {
        Ok(self.str_rust_data_type(conn)? == column.str_rust_data_type(conn)?)
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
        if let Ok(geometry) = self.geometry(conn) {
            return geometry.rust_type(self.is_nullable());
        }
        if let Ok(geography) = self.geography(conn) {
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
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, WebCodeGenError> {
        use crate::schema::tables;
        tables::table
            .filter(tables::table_name.eq(&self.table_name))
            .filter(tables::table_schema.eq(&self.table_schema))
            .filter(tables::table_catalog.eq(&self.table_catalog))
            .first::<Table>(conn)
            .map_err(WebCodeGenError::from)
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
        if let Ok(geometry) = self.geometry(conn) {
            return Ok(geometry.supports_copy());
        }
        if let Ok(geography) = self.geography(conn) {
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
        if self.geometry(conn).is_ok() || self.geography(conn).is_ok() {
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
        if self.geometry(conn).is_ok() || self.geography(conn).is_ok() {
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
        if self.geometry(conn).is_ok() || self.geography(conn).is_ok() {
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
    /// Returns whether the column is automatically generated
    ///
    /// A column is automatically generated if:
    /// - it is marked as `ALWAYS` generated
    /// - it has a default value that starts with `nextval`
    /// - it has a default value that starts with `CURRENT_TIMESTAMP`
    /// - it is an identity column
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether the column is automatically generated
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
            && self.foreign_keys(conn)?.into_iter().any(|key| {
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
            && self.foreign_keys(conn)?.into_iter().any(|key| {
                let Ok(foreign_columns) = key.foreign_columns(conn) else {
                    return false;
                };

                foreign_columns.len() == 1
                    && foreign_columns[0].table_name == "users"
                    && foreign_columns[0].column_name == "id"
            }))
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

    /// Load all columns from the database
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of `Column` if the operation was
    /// successful,
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::columns;
        columns::table.load::<Column>(conn).map_err(WebCodeGenError::from)
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

    /// Returns whether the column is a foreign key
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn is_foreign_key(&self, conn: &mut PgConnection) -> Result<bool, WebCodeGenError> {
        use crate::schema::{key_column_usage, referential_constraints};
        Ok(key_column_usage::table
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
            .filter(key_column_usage::column_name.eq(&self.column_name))
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .select(KeyColumnUsage::as_select())
            .first::<KeyColumnUsage>(conn)
            .optional()?
            .is_some())
    }

    /// Returns the foreign table of the column if it is a foreign key.
    /// If the column is not a foreign key, returns `None`.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Option` of a tuple containing the foreign
    /// table and the foreign column if the operation was successful, or a
    /// `DieselError` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn foreign_keys(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<KeyColumnUsage>, DieselError> {
        use crate::schema::{
            columns, constraint_column_usage, key_column_usage, table_constraints, tables,
        };
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
            .filter(table_constraints::table_name.eq(&self.table_name))
            .filter(table_constraints::table_schema.eq(&self.table_schema))
            .filter(table_constraints::table_catalog.eq(&self.table_catalog))
            .filter(key_column_usage::column_name.eq(&self.column_name))
            .order_by(key_column_usage::constraint_name.asc())
            .select(KeyColumnUsage::as_select())
            .distinct()
            .load::<KeyColumnUsage>(conn)
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
            .filter(key_column_usage::column_name.eq(&self.column_name))
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .filter(referential_constraints::delete_rule.eq("CASCADE"))
            .select(KeyColumnUsage::as_select())
            .first::<KeyColumnUsage>(conn)
            .is_ok()
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
    /// - the column is not nullable
    /// - the table that contains the column, L, is an extension table
    /// - the column is part of a foreign key to a table T
    /// - the foreign key refers to the primary key of T
    /// - the column is ALSO part of a composite foreign key (CFK) to T
    /// - the CFK is composed of the column and the primary key of L
    pub fn requires_partial_builder(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<KeyColumnUsage>, WebCodeGenError> {
        if self.is_nullable() {
            // If the column is nullable, we do not need a partial builder
            return Ok(None);
        }
        // First, we retrieve the table that contains the column
        let table = self.table(conn)?;
        // Then, we check if the table is an extension table
        if table.extension_table(conn)?.is_none() {
            return Ok(None);
        }
        // Next, we retrieve the foreign keys of the column
        let foreign_keys = self.foreign_keys(conn)?;
        // If there are no foreign keys, we return false
        if foreign_keys.is_empty() {
            return Ok(None);
        }
        // We check whether there exist a single column foreign key
        // which refers to the primary key of the foreign table.
        let mut single_column_fk_to_pk = None;
        for foreign_key in &foreign_keys {
            if foreign_key.is_foreign_primary_key(conn)?
                && foreign_key.number_of_columns(conn)? == 1
            {
                // If we have already found a single column foreign key to the
                // primary key of the foreign table, we raise a unimplemented error
                // as it is unclear how to handle this case.
                if single_column_fk_to_pk.is_some() {
                    unimplemented!(
                        "Column \"{}\".\"{}\" has multiple single-column foreign keys to the primary key of the foreign table. This is not supported yet.",
                        self.table_name,
                        self.column_name
                    );
                }

                single_column_fk_to_pk = Some(foreign_key);
            }
        }

        let Some(single_column_fk_to_pk) = single_column_fk_to_pk else {
            // If we have not found a single column foreign key to the primary key of the
            // foreign table, we return false as we cannot handle this case.
            return Ok(None);
        };

        // Now, after having identified the single column foreign key to the primary key
        // of the foreign table, we try to identify a composite foreign key
        // which is composed of the column and the primary key of the table that
        // contains the column.
        let primary_key_columns = table.primary_key_columns(conn)?;
        let Some(foreign_table) = single_column_fk_to_pk.foreign_table(conn)? else {
            return Ok(None);
        };

        // We expect the composite foreign key to be composed of the column
        // and the primary key of the table that contains the column.
        let mut expected_foreign_columns = vec![self.clone()];
        expected_foreign_columns.extend(primary_key_columns);

        let mut found_composite_fk = None;

        for foreign_key in &foreign_keys {
            if foreign_key == single_column_fk_to_pk {
                // We skip the single column foreign key to the primary key of the foreign table
                continue;
            }
            if foreign_key.foreign_table(conn)?.as_ref() != Some(&foreign_table) {
                // We skip foreign keys which do not refer to the same foreign table
                continue;
            }
            let columns = foreign_key.columns(conn)?;
            if columns == expected_foreign_columns {
                if found_composite_fk.is_some() {
                    // If we have already found a composite foreign key, we raise a
                    // unimplemented error as it is unclear how to handle this case.
                    unimplemented!(
                        "Column \"{}\".\"{}\" has multiple composite foreign keys to the foreign table \"{}\". This is not supported yet: {foreign_keys:#?}",
                        self.table_name,
                        self.column_name,
                        foreign_table.table_name
                    );
                }

                found_composite_fk = Some(foreign_key.clone());
            }
        }

        Ok(found_composite_fk)
    }

    /// Returns the `same-as` UNIQUE constraints for the column, if any.
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
    /// The `same-as` constraints are defined as:
    ///
    /// * A column is a `same-as` constraint if is part of a `composite` foreign
    ///   key which refers to a "tautological" composite UNIQUE constraint which
    ///   is composed of the primary key of the foreign table and the column
    ///   itself. An important corner case to take into account is that the
    ///   foreign table's composite UNIQUE constraint may be the COMPOSITE
    ///   PRIMARY KEY of the foreign table itself, and not the primary key plus
    ///   something else.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `Vec` of tuples, each containing a
    /// `KeyColumnUsage` and the foreign `Column` that the current column is
    /// a `same-as` constraint for.
    pub fn same_as_constraints(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Vec<(KeyColumnUsage, Column)>, WebCodeGenError> {
        let foreign_keys = self.foreign_keys(conn)?;
        let mut same_as_constraints = Vec::new();
        for foreign_key in foreign_keys {
            // First, we check whether the foreign key is a composite foreign key
            if !foreign_key.is_composite(conn)? {
                // If the foreign key is not a composite foreign key, we skip it
                continue;
            }

            // Next, we check whether the foreign key is referring to a UNIQUE constraint
            if !foreign_key.is_foreign_unique_key(conn)? {
                // If the foreign key is not referring to a UNIQUE constraint, we skip it
                continue;
            };

            let foreign_key_columns = foreign_key.foreign_columns(conn)?;
            // We identify the foreign column curresponding to the current column.
            let Some(foreign_column) =
                foreign_key.columns(conn)?.into_iter().zip(foreign_key_columns.iter()).find_map(
                    |(col, foreign_col)| {
                        if col.column_name == self.column_name { Some(foreign_col) } else { None }
                    },
                )
            else {
                // If we cannot find the foreign column corresponding to the current column,
                // Something is wrong with the current implementation and we panic.
                panic!(
                    "Column \"{}\".\"{}\" is part of a composite foreign key which refers to a UNIQUE constraint, but the foreign column could not be found.",
                    self.table_name, self.column_name
                );
            };

            // Finally, if the UNIQUE constraint is composed of the primary key
            // of the foreign table and the foreign column
            // associated with the current column, we consider it a `same-as`
            // constraint.
            let Some(foreign_table) = foreign_key.foreign_table(conn)? else {
                unreachable!(
                    "Column \"{}\".\"{}\" is part of a composite foreign key which refers to a UNIQUE constraint, but the foreign table could not be found.",
                    self.table_name, self.column_name
                );
            };

            let mut expected_foreign_columns = foreign_table.primary_key_columns(conn)?;

            // If the foreign column is not part of the foreign table's primary key, we add
            // it
            if !expected_foreign_columns.contains(&foreign_column) {
                expected_foreign_columns.push(foreign_column.clone());
            }

            // If the foreign key's foreign columns are not the same as the expected foreign
            // columns, then this constraint is not a `same-as` constraint.
            if foreign_key_columns != expected_foreign_columns {
                continue;
            }

            // Otherwise, we have identified a `same-as` constraint and we add it to the
            // list
            same_as_constraints.push((foreign_key, foreign_column.clone()));
        }

        Ok(same_as_constraints)
    }
}
