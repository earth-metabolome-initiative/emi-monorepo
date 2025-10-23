//! Submodule providing the `ColumnMetadata` struct for a [`Column`] model.

use std::rc::Rc;

use crate::models::{PgDescription, PgType, Table};

#[derive(Clone, Debug)]
/// Rich metadata about a PostgreSQL table column.
///
/// This struct wraps a column model with additional metadata loaded from
/// related system catalog tables, including:
/// - The table that owns the column
/// - The resolved PostgreSQL type ([`PgType`]) for the column
/// - Column description from `pg_catalog.pg_description`
///
/// This metadata is constructed during
/// [`PgDatabase`](crate::database::PgDatabase) building and cached for
/// efficient access via the [`ColumnLike`](sql_traits::traits::ColumnLike)
/// trait.
///
/// ## Type Resolution
///
/// The `pg_type` field contains the fully resolved type information from
/// `pg_catalog.pg_type`, not just the type name from
/// `information_schema.columns`. This provides access to:
/// - Internal type name (`typname`)
/// - Type category and properties
/// - Array element types for array types
pub struct ColumnMetadata {
    /// The table the column belongs to.
    table: Rc<Table>,
    /// The description of the column, if any.
    description: Option<PgDescription>,
    /// The associated PgType.
    pg_type: PgType,
}

impl ColumnMetadata {
    /// Creates a new `ColumnMetadata` instance.
    pub fn new(table: Rc<Table>, description: Option<PgDescription>, pg_type: PgType) -> Self {
        Self { table, description, pg_type }
    }

    /// Returns the table the column belongs to.
    pub fn table(&self) -> &Table {
        self.table.as_ref()
    }

    /// Returns the description of the column, if any.
    pub fn description(&self) -> Option<&PgDescription> {
        self.description.as_ref()
    }

    /// Returns the associated PgType.
    pub fn pg_type(&self) -> &PgType {
        &self.pg_type
    }

    /// Returns the normalized data type of the column.
    pub fn normalized_data_type(&self) -> String {
        self.pg_type.typname.clone()
    }
}
