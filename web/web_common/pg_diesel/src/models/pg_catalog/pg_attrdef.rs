//! PostgreSQL column default values catalog model.
//!
//! This module provides the `PgAttrdef` struct for working with the
//! `pg_catalog.pg_attrdef` system catalog table.

/// Represents a row from the `pg_catalog.pg_attrdef` table.
///
/// Contains default expressions for table columns that have defaults.
/// This table stores the default value expressions that are applied when
/// new rows are inserted without specifying values for certain columns.
#[derive(diesel::Queryable, diesel::QueryableByName, diesel::Selectable)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[diesel(table_name = crate::schema::pg_catalog::pg_attrdef::pg_attrdef)]
pub struct PgAttrdef {
    /// OID of this entry (primary key).
    pub oid: u32,
    /// OID of the table this default belongs to.
    pub adrelid: u32,
    /// Column number that this default is for.
    pub adnum: i16,
    /// Default expression (in nodeToString representation).
    pub adbin: String,
}
