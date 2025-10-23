use std::{fmt::Display, rc::Rc};

use diesel::{OptionalExtension, PgConnection, Queryable, QueryableByName, Selectable};

mod cached_queries;

use super::check_constraint::CheckConstraint;
use crate::{
    model_metadata::ColumnMetadata,
    models::{GeographyColumn, GeometryColumn, KeyColumnUsage, PgType, Table},
};

/// Struct defining the `information_schema.columns` table.
#[derive(
    Queryable, QueryableByName, Selectable, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::information_schema::columns::columns)]
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
    pub __is_nullable: String,
    /// Data type of the column
    data_type: String,
    /// Maximum length of the character data type
    pub character_maximum_length: Option<i32>,
    /// Maximum length in bytes of the character data type
    pub character_octet_length: Option<i32>,
    /// Catalog containing the character set of the column
    pub character_set_catalog: Option<String>,
    /// Schema containing the character set of the column
    pub character_set_schema: Option<String>,
    /// Name of the character set of the column
    pub character_set_name: Option<String>,
    /// Catalog containing the collation of the column
    pub collation_catalog: Option<String>,
    /// Schema containing the collation of the column
    pub collation_schema: Option<String>,
    /// Name of the collation of the column
    pub collation_name: Option<String>,
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
    /// Name of the UDT; `NULL` if not applicable.
    pub udt_name: Option<String>,
    /// Catalog of the domain type
    pub domain_catalog: Option<String>,
    /// Schema of the domain type
    pub domain_schema: Option<String>,
    /// Name of the domain type
    pub domain_name: Option<String>,
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
    /// "YES" if the column is an identity column; "NO" otherwise.
    pub is_identity: Option<String>,
    /// "ALWAYS" or "BY DEFAULT" for identity columns
    pub identity_generation: Option<String>,
    /// Start value of the identity sequence
    pub identity_start: Option<String>,
    /// Increment of the identity sequence
    pub identity_increment: Option<String>,
    /// Maximum value of the identity sequence
    pub identity_maximum: Option<String>,
    /// Minimum value of the identity sequence
    pub identity_minimum: Option<String>,
    /// "YES" if identity sequence cycles, "NO" otherwise
    pub identity_cycle: Option<String>,
    /// "ALWAYS", "BY DEFAULT", or empty string for computed/generated columns.
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
    /// Returns the metadata of the column.
    ///
    /// # Arguments
    ///
    /// * `table` - The table the column belongs to.
    /// * `conn` - A mutable reference to a `PgConnection`
    ///
    /// # Errors
    ///
    /// * If an error occurs while querying the database
    pub fn metadata(
        &self,
        table: Rc<Table>,
        conn: &mut PgConnection,
    ) -> Result<ColumnMetadata, diesel::result::Error> {
        Ok(ColumnMetadata::new(
            table,
            cached_queries::pg_description(self, conn).optional()?,
            self.pg_type(conn)?,
        ))
    }

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
    ) -> Result<Vec<CheckConstraint>, diesel::result::Error> {
        cached_queries::check_constraints(self, conn)
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
    ) -> Result<Option<GeometryColumn>, diesel::result::Error> {
        cached_queries::geometry_column(self, conn)
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
    ) -> Result<Option<GeographyColumn>, diesel::result::Error> {
        cached_queries::geography_column(self, conn)
    }

    /// Returns the data type associated with the column as a string.
    pub fn data_type_str(&self) -> &str {
        if self.has_custom_type() {
            if let Some(udt_name) = &self.udt_name {
                udt_name
            } else {
                unimplemented!(
                    "Column with USER-DEFINED type but no udt_name - Unclear how to handle this case"
                )
            }
        } else {
            &self.data_type
        }
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
    /// successful, or a `diesel::result::Error` if an error occurred
    ///
    /// # Errors
    ///
    /// If an error occurs while querying the database
    pub fn pg_type(&self, conn: &mut PgConnection) -> Result<PgType, diesel::result::Error> {
        cached_queries::pg_type(self, conn)
    }

    #[must_use]
    /// Returns whether the column has a custom type
    pub fn has_custom_type(&self) -> bool {
        self.data_type == "USER-DEFINED"
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
    pub fn table(&self, conn: &mut PgConnection) -> Result<Table, diesel::result::Error> {
        cached_queries::table(self, conn)
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
    pub fn is_unique(&self, conn: &mut PgConnection) -> Result<bool, diesel::result::Error> {
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

    /// Returns whether the column has a date time type.
    pub fn has_datetime_type(&self) -> bool {
        self.data_type.contains("timestamp")
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
    ) -> Result<Vec<KeyColumnUsage>, diesel::result::Error> {
        cached_queries::foreign_keys(self, conn)
    }

    /// Returns whether the column has foreign keys.
    ///
    /// # Arguments
    ///
    /// * `conn` - A mutable reference to a `PgConnection`
    pub fn has_foreign_keys(&self, conn: &mut PgConnection) -> bool {
        self.foreign_keys(conn).is_ok_and(|keys| !keys.is_empty())
    }
}
