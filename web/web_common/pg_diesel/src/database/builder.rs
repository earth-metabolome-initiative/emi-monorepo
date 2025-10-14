//! Submodule providing a builder and its trait implementation for constructing
//! a [`PgDatabase`](crate::database::PgDatabase) instance from a PostgreSQL
//! connection and relevant schema information.

use std::{fmt::Display, rc::Rc};

use common_traits::{
    builder::{Attributed, IsCompleteBuilder},
    prelude::{Builder, BuilderError},
};
use diesel::PgConnection;
use sql_traits::traits::TableLike;

use crate::{PgDatabase, database::KeyColumnUsageMetadata, models::Table};

#[derive(Default)]
/// Builder for constructing a `PgDatabase` instance.
pub struct PgDatabaseBuilder<'conn> {
    /// Connection to the PostgreSQL database.
    connection: Option<&'conn mut PgConnection>,
    /// The catalog (database) name to filter by.
    catalog: Option<String>,
    /// The schema names to include.
    schemas: Vec<String>,
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
}

impl Display for PgDatabaseAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgDatabaseAttribute::Connection => write!(f, "connection"),
            PgDatabaseAttribute::Catalog => write!(f, "catalog"),
            PgDatabaseAttribute::Schemas => write!(f, "schemas"),
        }
    }
}

#[derive(Debug)]
pub enum PgDatabaseBuildError {
    /// An error occurred while building the `PgDatabase`.
    Builder(BuilderError<PgDatabaseAttribute>),
    /// An error occurred while querying the database schema.
    Diesel(diesel::result::Error),
}

impl Display for PgDatabaseBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PgDatabaseBuildError::Builder(err) => write!(f, "Builder error: {}", err),
            PgDatabaseBuildError::Diesel(err) => write!(f, "Diesel error: {}", err),
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
        }
    }
}

impl<'conn> PgDatabaseBuilder<'conn> {
    /// Sets the PostgreSQL connection to use for building the `PgDatabase`.
    pub fn connection(mut self, connection: &'conn mut PgConnection) -> Self {
        self.connection = Some(connection);
        self
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
        let mut foreign_keys = Vec::new();
        let mut unique_indices = Vec::new();
        let mut columns = Vec::new();
        let mut tables_with_metadata = Vec::new();
        for table in tables {
            let table_metadata = table.metadata(connection)?;
            for column in table_metadata.column_rcs() {
                columns.push((column.clone(), table.clone()));
            }
            for fk in table_metadata.foreign_key_rcs() {
                let metadata = KeyColumnUsageMetadata::new(&fk, connection)?;
                foreign_keys.push((fk.clone(), metadata));
            }
            for index in table_metadata.unique_index_rcs() {
                let metadata = index.metadata(table.clone(), connection)?;
                unique_indices.push((index.clone(), metadata));
            }
            tables_with_metadata.push((table, table_metadata));
        }

        Ok(PgDatabase::new(tables_with_metadata, columns, unique_indices, foreign_keys))
    }
}
