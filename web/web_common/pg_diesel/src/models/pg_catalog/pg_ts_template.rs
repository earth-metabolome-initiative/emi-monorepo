//! Model for pg_catalog.pg_ts_template table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ts_template::pg_ts_template)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_ts_template` catalog table.
pub struct PgTsTemplate {
    /// Row identifier
    pub oid: u32,
    /// Text search template name
    pub tmplname: String,
    /// The OID of the namespace that contains this template
    pub tmplnamespace: u32,
    /// OID of the template's initialization function
    pub tmplinit: u32,
    /// OID of the template's lexize function
    pub tmpllexize: u32,
}
