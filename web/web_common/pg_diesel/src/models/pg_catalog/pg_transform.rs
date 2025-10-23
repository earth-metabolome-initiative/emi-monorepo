//! Model for pg_catalog.pg_transform table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_transform::pg_transform)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_transform` catalog table.
pub struct PgTransform {
    /// Row identifier
    pub oid: u32,
    /// OID of the data type this transform is for
    pub trftype: u32,
    /// OID of the language this transform is for
    pub trflang: u32,
    /// OID of the function to transform from SQL to the language
    pub trffromsql: u32,
    /// OID of the function to transform from the language to SQL
    pub trftosql: u32,
}
