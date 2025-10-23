//! Model for pg_catalog.pg_ts_parser table.

use diesel::prelude::*;

#[derive(
    Queryable, QueryableByName, Selectable, Identifiable, Debug, PartialEq, Eq, Clone, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[diesel(table_name = crate::schema::pg_catalog::pg_ts_parser::pg_ts_parser)]
#[diesel(primary_key(oid))]
/// Represents a row from the `pg_ts_parser` catalog table.
pub struct PgTsParser {
    /// Row identifier
    pub oid: u32,
    /// Text search parser name
    pub prsname: String,
    /// The OID of the namespace that contains this parser
    pub prsnamespace: u32,
    /// OID of the parser's startup function
    pub prsstart: u32,
    /// OID of the parser's next-token function
    pub prstoken: u32,
    /// OID of the parser's shutdown function
    pub prsend: u32,
    /// OID of the parser's headline function
    pub prsheadline: u32,
    /// OID of the parser's lextype function
    pub prslextype: u32,
}
