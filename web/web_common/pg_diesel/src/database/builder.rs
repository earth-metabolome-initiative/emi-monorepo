//! Builder pattern for constructing a [`PgDatabase`] instance.

use std::rc::Rc;

use diesel::PgConnection;
use sql_traits::{structs::generic_db::GenericDBBuilder, traits::TableLike};

use crate::{
    PgDatabase,
    models::{PgProc, Table},
};

#[derive(Default)]
/// Builder for constructing a [`PgDatabase`] instance from PostgreSQL metadata.
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

#[derive(Debug, thiserror::Error)]
/// Errors that can occur when building a [`PgDatabase`] instance.
///
/// This error type encompasses all failure modes during database metadata
/// loading:
/// - Missing required builder attributes
/// - Database query failures
/// - Invalid denylist configurations
pub enum PgDatabaseBuildError {
    #[error("Missing required builder attribute: {0}")]
    /// An attribute was missing.
    MissingAttribute(&'static str),
    #[error("Diesel error: {0}")]
    /// An error occurred while querying the database schema.
    Diesel(#[from] diesel::result::Error),
    #[error("Duplicate denylisted type: {0}")]
    /// A deny-listed type was inserted multiple times.
    DuplicateDenylistedType(String),
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

impl TryFrom<PgDatabaseBuilder<'_>> for PgDatabase {
    type Error = PgDatabaseBuildError;

    fn try_from(value: PgDatabaseBuilder<'_>) -> Result<Self, Self::Error> {
        let connection =
            value.connection.ok_or(PgDatabaseBuildError::MissingAttribute("connection"))?;

        let table_catalog =
            value.catalog.ok_or(PgDatabaseBuildError::MissingAttribute("catalog"))?;

        let table_schemas = {
            if value.schemas.is_empty() {
                return Err(PgDatabaseBuildError::MissingAttribute("schemas"));
            } else {
                value.schemas
            }
        };

        let mut generic_builder = GenericDBBuilder::new(table_catalog.clone());

        for function in PgProc::load_all(connection)? {
            let metadata = crate::database::PgProcMetadata::new(&function, connection)?;
            generic_builder = generic_builder.add_function(std::rc::Rc::new(function), metadata);
        }

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
            let table_metadata = table.metadata(connection, &value.denylist_types)?;

            for check_constraint in table_metadata.check_constraint_rcs() {
                let metadata = check_constraint.metadata(
                    table.clone(),
                    &table_metadata,
                    generic_builder.function_rc_vec().as_slice(),
                    connection,
                )?;
                generic_builder =
                    generic_builder.add_check_constraint(check_constraint.clone(), metadata);
            }

            for column in table_metadata.column_rcs() {
                generic_builder = generic_builder
                    .add_column(column.clone(), column.metadata(table.clone(), connection)?);
            }

            for fk in table_metadata.foreign_key_rcs() {
                generic_builder = generic_builder
                    .add_foreign_key(fk.clone(), fk.metadata(table.clone(), connection)?);
            }

            for index in table_metadata.unique_index_rcs() {
                generic_builder = generic_builder
                    .add_unique_index(index.clone(), index.metadata(table.clone(), connection)?);
            }

            generic_builder = generic_builder.add_table(table, table_metadata);
        }

        Ok(generic_builder.into())
    }
}
