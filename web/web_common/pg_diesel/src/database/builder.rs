//! Builder pattern for constructing a [`PgDatabase`] instance.
//!
//! This module provides [`PgDatabaseBuilder`], which implements a type-safe
//! builder pattern for loading PostgreSQL metadata from system catalogs into a
//! [`PgDatabase`] instance.
//!
//! ## Builder Pattern
//!
//! The builder requires three essential attributes:
//! - A database connection ([`PgConnection`])
//! - A catalog (database) name to query
//! - One or more schema names to include
//!
//! ## Type Denylisting
//!
//! The builder supports denylisting PostgreSQL types that cannot be mapped to
//! Diesel types (e.g., `anyarray`, `pg_ndistinct`). Denylisted types are
//! excluded from column generation.
//!
//! ## Error Handling
//!
//! The builder can fail with [`PgDatabaseBuildError`] which wraps:
//! - Builder errors (missing required attributes)
//! - Diesel errors (database query failures)
//! - Duplicate denylist type errors
//!
//! ## Example
//!
//! ```ignore
//! use pg_diesel::database::PgDatabaseBuilder;
//!
//! let db = PgDatabaseBuilder::default()
//!     .connection(&mut conn)
//!     .catalog("my_database")
//!     .schemas(vec!["public".to_owned(), "information_schema".to_owned()])
//!     .denylist_types(["anyarray", "pg_ndistinct"])?
//!     .build()?;
//! ```

use std::{fmt::Display, rc::Rc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use diesel::PgConnection;
use sql_traits::{structs::generic_db::GenericDBBuilder, traits::TableLike};

use crate::{
    PgDatabase,
    models::{PgProc, Table},
};

#[derive(Default)]
/// Builder for constructing a [`PgDatabase`] instance from PostgreSQL metadata.
///
/// This builder follows the type-state pattern to ensure all required
/// attributes are provided before building. It queries the PostgreSQL system
/// catalogs to load complete metadata about tables, columns, constraints,
/// indexes, and functions.
///
/// ## Required Attributes
///
/// - **connection**: A PostgreSQL database connection for querying metadata
/// - **catalog**: The database name to query (typically the current database)
/// - **schemas**: One or more schema names to include (e.g., "public",
///   "pg_catalog")
///
/// ## Optional Attributes
///
/// - **denylist_types**: PostgreSQL type names to exclude from column
///   generation
///
/// ## Usage
///
/// ```ignore
/// let db = PgDatabaseBuilder::default()
///     .connection(&mut conn)
///     .catalog("mydb")
///     .add_schema("public")
///     .add_schema("information_schema")
///     .build()?;
/// ```
pub struct PgDatabaseBuilder<'conn> {
    /// Connection to the PostgreSQL database.
    connection: Option<&'conn mut PgConnection>,
    /// The catalog (database) name to filter by.
    catalog: Option<String>,
    /// The schema names to include.
    schemas: Vec<String>,
    /// Types denylist.
    denylist_types: Vec<String>,
}

#[derive(Debug)]
/// Attributes that can be set on the `PgDatabaseBuilder`.
pub enum PgDatabaseAttribute {
    /// A PostgreSQL database connection.
    Connection,
    /// The catalog (database) name to filter by.
    Catalog,
    /// The schema names to include.
    Schemas,
    /// Types denylist.
    DenylistTypes,
}

impl Display for PgDatabaseAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgDatabaseAttribute::Connection => write!(f, "connection"),
            PgDatabaseAttribute::Catalog => write!(f, "catalog"),
            PgDatabaseAttribute::Schemas => write!(f, "schemas"),
            PgDatabaseAttribute::DenylistTypes => write!(f, "denylist_types"),
        }
    }
}

#[derive(Debug)]
/// Errors that can occur when building a [`PgDatabase`] instance.
///
/// This error type encompasses all failure modes during database metadata
/// loading:
/// - Missing required builder attributes
/// - Database query failures
/// - Invalid denylist configurations
pub enum PgDatabaseBuildError {
    /// An error occurred while building the `PgDatabase`.
    Builder(BuilderError<PgDatabaseAttribute>),
    /// An error occurred while querying the database schema.
    Diesel(diesel::result::Error),
    /// A deny-listed type was inserted multiple times.
    DuplicateDenylistedType(String),
}

impl Display for PgDatabaseBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgDatabaseBuildError::Builder(err) => write!(f, "Builder error: {}", err),
            PgDatabaseBuildError::Diesel(err) => write!(f, "Diesel error: {}", err),
            PgDatabaseBuildError::DuplicateDenylistedType(ty) => {
                write!(f, "Duplicate deny-listed type: {}", ty)
            }
        }
    }
}

impl From<BuilderError<PgDatabaseAttribute>> for PgDatabaseBuildError {
    fn from(err: BuilderError<PgDatabaseAttribute>) -> Self {
        PgDatabaseBuildError::Builder(err)
    }
}

impl From<diesel::result::Error> for PgDatabaseBuildError {
    fn from(err: diesel::result::Error) -> Self {
        PgDatabaseBuildError::Diesel(err)
    }
}

impl std::error::Error for PgDatabaseBuildError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            PgDatabaseBuildError::Builder(err) => Some(err),
            PgDatabaseBuildError::Diesel(err) => Some(err),
            PgDatabaseBuildError::DuplicateDenylistedType(_) => None,
        }
    }
}

impl<'conn> PgDatabaseBuilder<'conn> {
    /// Sets the PostgreSQL connection to use for building the `PgDatabase`.
    pub fn connection(mut self, connection: &'conn mut PgConnection) -> Self {
        self.connection = Some(connection);
        self
    }

    /// Sets the catalog (database) name to filter by.
    pub fn catalog<S: ToString>(mut self, catalog: S) -> Self {
        self.catalog = Some(catalog.to_string());
        self
    }

    /// Adds a schema name to include.
    pub fn add_schema<S: ToString>(mut self, schema: S) -> Self {
        self.schemas.push(schema.to_string());
        self
    }

    /// Sets the schema names to include.
    pub fn schemas(mut self, schemas: Vec<String>) -> Self {
        self.schemas = schemas;
        self
    }

    /// Adds a type to the denylist.
    pub fn denylist_type<S: ToString>(mut self, ty: S) -> Result<Self, PgDatabaseBuildError> {
        let ty_str = ty.to_string();
        if self.denylist_types.contains(&ty_str) {
            return Err(PgDatabaseBuildError::DuplicateDenylistedType(ty_str));
        }
        self.denylist_types.push(ty_str);
        Ok(self)
    }

    /// Adds multiple types to the denylist.
    pub fn denylist_types<I, S>(mut self, types: I) -> Result<Self, PgDatabaseBuildError>
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        for ty in types {
            self = self.denylist_type(ty)?;
        }
        Ok(self)
    }
}

impl Attributed for PgDatabaseBuilder<'_> {
    type Attribute = PgDatabaseAttribute;
}

impl IsCompleteBuilder for PgDatabaseBuilder<'_> {
    fn is_complete(&self) -> bool {
        self.connection.is_some() && self.catalog.is_some() && !self.schemas.is_empty()
    }
}

impl Builder for PgDatabaseBuilder<'_> {
    type Error = PgDatabaseBuildError;
    type Object = PgDatabase;

    fn build(self) -> Result<Self::Object, Self::Error> {
        let connection = self
            .connection
            .ok_or(BuilderError::IncompleteBuild(PgDatabaseAttribute::Connection))?;

        let table_catalog =
            self.catalog.ok_or(BuilderError::IncompleteBuild(PgDatabaseAttribute::Catalog))?;

        let table_schemas = {
            if self.schemas.is_empty() {
                return Err(BuilderError::IncompleteBuild(PgDatabaseAttribute::Schemas).into());
            } else {
                self.schemas
            }
        };

        let mut builder = GenericDBBuilder::new().catalog_name(table_catalog.clone());

        let mut tables = Vec::new();
        for table_schema in &table_schemas {
            tables.extend(
                Table::load_all(connection, &table_catalog, table_schema)?.into_iter().map(Rc::new),
            );
        }

        // We sort the tables by schema and name to enable efficient binary search
        // later.
        tables.sort_by_key(|table| {
            (table.as_ref().table_schema().unwrap_or("").to_owned(), table.table_name().to_owned())
        });

        // For each table, we determine all of the foreign keys and for each foreign key
        // we determine which table it references.
        for table in tables {
            let table_metadata = table.metadata(connection, &self.denylist_types)?;

            for column in table_metadata.column_rcs() {
                builder =
                    builder.add_column(column.clone(), column.metadata(table.clone(), connection)?);
            }

            for fk in table_metadata.foreign_key_rcs() {
                builder =
                    builder.add_foreign_key(fk.clone(), fk.metadata(table.clone(), connection)?);
            }

            for index in table_metadata.unique_index_rcs() {
                builder = builder
                    .add_unique_index(index.clone(), index.metadata(table.clone(), connection)?);
            }

            builder = builder.add_table(table, table_metadata);
        }

        for function in PgProc::load_all(connection)? {
            let metadata = crate::database::PgProcMetadata::new(&function, connection)?;
            builder = builder.add_function(std::rc::Rc::new(function), metadata);
        }

        Ok(builder.build().expect("Failed to build PgDatabase: catalog_name should be set"))
    }
}
