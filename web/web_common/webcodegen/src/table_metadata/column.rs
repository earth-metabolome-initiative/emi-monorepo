use crate::errors::WebCodeGenError;
use crate::table_metadata::pg_type::postgres_type_to_diesel;
use crate::Table;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, QueryDsl, Queryable, QueryableByName,
    RunQueryDsl, Selectable, SelectableHelper,
};
use snake_case_sanitizer::Sanitizer as SnakeCaseSanizer;
use syn::{Ident, Type};

use crate::KeyColumnUsage;

use super::pg_type::{rust_type_str, PgType};
use super::table::{RESERVED_DIESEL_WORDS, RESERVED_RUST_WORDS};

/// Struct defining the `information_schema.columns` table.
#[derive(Queryable, QueryableByName, Selectable, PartialEq, Eq, Debug, Clone)]
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
    /// Ordinal position of the column within the table (count starts at 1)
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
    /// Catalog name of the character set
    pub character_set_catalog: Option<String>,
    /// Schema name of the character set
    pub character_set_schema: Option<String>,
    /// Name of the character set
    pub character_set_name: Option<String>,
    /// Catalog name of the collation
    pub collation_catalog: Option<String>,
    /// Schema name of the collation
    pub collation_schema: Option<String>,
    /// Name of the collation
    pub collation_name: Option<String>,
    /// Catalog name of the domain
    pub domain_catalog: Option<String>,
    /// Schema name of the domain
    pub domain_schema: Option<String>,
    /// Name of the domain
    pub domain_name: Option<String>,
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
    /// Generation expression of the identity column
    pub identity_generation: Option<String>,
    /// Start value of the identity column
    pub identity_start: Option<String>,
    /// Increment value of the identity column
    pub identity_increment: Option<String>,
    /// Maximum value of the identity column
    pub identity_maximum: Option<String>,
    /// Minimum value of the identity column
    pub identity_minimum: Option<String>,
    /// Indicates if the identity column cycles
    pub identity_cycle: Option<String>,
    /// Indicates if the column is generated ("ALWAYS" or "NEVER")
    pub is_generated: String,
    /// Generation expression of the column
    pub generation_expression: Option<String>,
    /// Indicates if the column is updatable ("YES" or "NO")
    pub is_updatable: String,
}

impl Column {
    /// Returns the raw data type of the column
    pub fn raw_data_type(&self) -> &str {
        &self.data_type
    }

    /// Returns the data type associated with the column as repo
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the data type of the column if the operation was successful,
    /// or a `WebCodeGenError` if an error occurred
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
    pub fn data_type(&self, conn: &mut PgConnection) -> Result<Type, WebCodeGenError> {
        let rust_type = if self.has_custom_type() {
            PgType::from_name(self.data_type_str(conn)?, conn)?.camelcased_name()
        } else {
            rust_type_str(self.data_type_str(conn)?).to_string()
        };

        let rust_type = if self.is_nullable() {
            format!("Option<{rust_type}>")
        } else {
            rust_type.to_string()
        };

        Ok(syn::parse_str(&rust_type).unwrap())
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
        let sanitizer = SnakeCaseSanizer::default()
            .include_defaults()
            .remove_leading_underscores()
            .remove_trailing_underscores();
        Ok(sanitizer.to_snake_case(&self.column_name)?)
    }

    /// Returns the sanitized snake case syn Ident of the table.
    /// 
    /// If the column name is a reserved diesel word, the returned ident will be prefixed with `__`.
    /// If the column name is a reserved rust word, the returned ident will be the raw ident.
    /// Otherwise, the returned ident will be the sanitized snake case ident.
    /// 
    /// # Returns
    /// 
    /// A `Result` containing the sanitized snake case `Ident` if the operation was successful,
    /// or a `WebCodeGenError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while sanitizing the column name
    pub fn sanitized_snake_case_ident(&self) -> Result<Ident, WebCodeGenError> {
        let snake_case_name = self.snake_case_name()?;
        if self.requires_diesel_sanitization()? {
            Ok(Ident::new(
                &format!("__{snake_case_name}"),
                proc_macro2::Span::call_site(),
            ))
        } else if RESERVED_RUST_WORDS.contains(&snake_case_name.as_str()) {
            Ok(Ident::new_raw(
                &snake_case_name,
                proc_macro2::Span::call_site(),
            ))
        } else {
            Ok(Ident::new(&snake_case_name, proc_macro2::Span::call_site()))
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
        if self.has_custom_type() {
            let name = PgType::from_name(self.data_type_str(conn)?, conn)?.camelcased_name();
            let name = if self.is_nullable() {
                format!("diesel::sql_types::Nullable<crate::Pg{name}>")
            } else {
                format!("crate::Pg{name}")
            };

            Ok(syn::parse_str(&name).unwrap())
        } else {
            Ok(postgres_type_to_diesel(
                self.data_type_str(conn)?,
                self.is_nullable(),
            ))
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
    /// 
    pub fn is_automatically_generated(&self) -> bool {
        self.is_generated == "ALWAYS"
            || self
                .column_default
                .as_ref()
                .map_or(false, |d| d.starts_with("nextval"))
            || self
                .column_default
                .as_ref()
                .map_or(false, |d| d.starts_with("CURRENT_TIMESTAMP"))
            || self.is_identity.as_ref().map_or(false, |i| i == "YES")
    }

    /// Returns whether the column contains the update user and is defined by the SESSION user
    pub fn is_updated_by(&self, conn: &mut PgConnection) -> bool {
        self.column_name == "updated_by" && self.is_session_user_generated(conn)
    }

    /// Returns whether the column contains the creation user and is defined by the SESSION user
    pub fn is_created_by(&self, conn: &mut PgConnection) -> bool {
        self.column_name == "created_by" && self.is_session_user_generated(conn)
    }

    /// Returns whether the a timestamp which has to be updated at each update operation
    pub fn is_updated_at(&self) -> bool {
        self.column_name == "updated_at" && self.data_type == "timestamp without time zone"
    }

    /// Returns whether the column is a session user generated column
    pub fn is_session_user_generated(&self, conn: &mut PgConnection) -> bool {
        // A column is associated to the user section if:
        // - it is a foreign key of the users table
        // - it is named `updated_by` or `created_by`

        if self.column_name != "updated_by" && self.column_name != "created_by" {
            return false;
        }

        if let Ok(Some((table, _))) = self.foreign_table(conn) {
            if table.table_name == "users" {
                return true;
            }
        }

        false
    }

    /// Load all columns from the database
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// 
    /// # Returns
    /// 
    /// A `Result` containing a `Vec` of `Column` if the operation was successful,
    /// 
    /// # Errors
    /// 
    /// If an error occurs while querying the database
    pub fn load_all(conn: &mut PgConnection) -> Result<Vec<Self>, WebCodeGenError> {
        use crate::schema::columns;
        columns::table.load::<Column>(conn).map_err(WebCodeGenError::from)
    }

    /// Returns whether the column is a foreign key
    /// 
    /// # Arguments
    /// 
    /// * `conn` - A mutable reference to a `PgConnection`
    /// 
    pub fn is_foreign_key(&self, conn: &mut PgConnection) -> bool {
        use crate::schema::key_column_usage;
        use crate::schema::referential_constraints;
        key_column_usage::table
            .inner_join(
                referential_constraints::table.on(
                    key_column_usage::constraint_name
                        .eq(referential_constraints::constraint_name)
                        .and(
                            key_column_usage::constraint_schema
                                .eq(referential_constraints::constraint_schema),
                        )
                        .and(
                            key_column_usage::constraint_catalog
                                .eq(referential_constraints::constraint_catalog),
                        ),
                ),
            )
            .filter(key_column_usage::column_name.eq(&self.column_name))
            .filter(key_column_usage::table_name.eq(&self.table_name))
            .filter(key_column_usage::table_schema.eq(&self.table_schema))
            .filter(key_column_usage::table_catalog.eq(&self.table_catalog))
            .select(KeyColumnUsage::as_select())
            .first::<KeyColumnUsage>(conn)
            .is_ok()
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
    /// A `Result` containing an `Option` of a tuple containing the foreign table and
    /// the foreign column if the operation was successful, or a `DieselError` if an error occurred
    /// 
    /// # Errors
    /// 
    /// If an error occurs while querying the database
    pub fn foreign_table(
        &self,
        conn: &mut PgConnection,
    ) -> Result<Option<(Table, Column)>, DieselError> {
        use crate::schema::columns;
        use crate::schema::constraint_column_usage;
        use crate::schema::key_column_usage;
        use crate::schema::table_constraints;
        use crate::schema::tables;
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
                    .and(
                        table_constraints::table_schema
                            .eq(key_column_usage::table_schema),
                    )
                    .and(
                        table_constraints::table_catalog
                            .eq(key_column_usage::table_catalog),
                    )),
            )
            .inner_join(
                constraint_column_usage::table
                    .on(constraint_column_usage::constraint_name
                        .eq(table_constraints::constraint_name)),
            )
            .inner_join(
                tables::table.on(tables::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(tables::table_schema.eq(constraint_column_usage::table_schema))
                    .and(
                        tables::table_catalog.eq(constraint_column_usage::table_catalog),
                    )),
            )
            .inner_join(
                columns::table.on(columns::table_name
                    .eq(constraint_column_usage::table_name)
                    .and(columns::table_schema.eq(constraint_column_usage::table_schema))
                    .and(
                        columns::table_catalog.eq(constraint_column_usage::table_catalog),
                    )
                    .and(columns::column_name.eq(constraint_column_usage::column_name))),
            )
            .filter(table_constraints::constraint_type.eq("FOREIGN KEY"))
            .filter(table_constraints::table_name.eq(&self.table_name))
            .filter(table_constraints::table_schema.eq(&self.table_schema))
            .filter(table_constraints::table_catalog.eq(&self.table_catalog))
            .filter(key_column_usage::column_name.eq(&self.column_name))
            .select((Table::as_select(), Column::as_select()))
            .first::<(Table, Column)>(conn)
            .map(Some)
            .or_else(|e| {
                if e == DieselError::NotFound {
                    Ok(None)
                } else {
                    Err(e)
                }
            })
    }
}
