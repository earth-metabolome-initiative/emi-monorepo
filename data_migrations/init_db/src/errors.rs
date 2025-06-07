//! Submodule providing the errors enumeration.

use csqlv::CSVSchemaError;
use diesel_migrations_utils::prelude::MigrationError;
use taxonomy_fetcher::{TaxonomyBuilderError, TaxonomyError, impls::ncbi::NCBITaxonEntry};
use webcodegen::errors::WebCodeGenError;

#[derive(Debug)]
#[allow(dead_code)]
/// Error enumeration for the `init_migration` module.
pub enum Error {
    /// Failed to establish database connection
    ConnectionFailed(diesel::ConnectionError),
    /// Failed to execute a query
    QueryFailed(diesel::result::Error),
    /// Consistency constraint check failed
    WebCodeGen(WebCodeGenError),
    /// Something failed while reading a file
    Io(std::io::Error),
    /// An error occurred while building the taxonomy
    TaxonomyBuilder(TaxonomyBuilderError<NCBITaxonEntry>),
    /// An error occurred while working with the taxonomy
    Taxonomy(TaxonomyError<u32>),
    /// An error occurred while executing a migration
    Migration(MigrationError),
    /// An error occurred while working with the CSV schema
    CSVSchema(CSVSchemaError),
}

impl From<diesel::ConnectionError> for Error {
    fn from(value: diesel::ConnectionError) -> Self {
        Error::ConnectionFailed(value)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(value: diesel::result::Error) -> Self {
        Error::QueryFailed(value)
    }
}

impl From<WebCodeGenError> for Error {
    fn from(value: WebCodeGenError) -> Self {
        Error::WebCodeGen(value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<TaxonomyError<u32>> for Error {
    fn from(value: TaxonomyError<u32>) -> Self {
        Error::Taxonomy(value)
    }
}

impl From<TaxonomyBuilderError<NCBITaxonEntry>> for Error {
    fn from(value: TaxonomyBuilderError<NCBITaxonEntry>) -> Self {
        Error::TaxonomyBuilder(value)
    }
}

impl From<MigrationError> for Error {
    fn from(value: MigrationError) -> Self {
        Error::Migration(value)
    }
}

impl From<CSVSchemaError> for Error {
    fn from(value: CSVSchemaError) -> Self {
        Error::CSVSchema(value)
    }
}
