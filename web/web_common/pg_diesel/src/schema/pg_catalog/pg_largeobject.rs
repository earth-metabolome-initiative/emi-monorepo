//! Submodule for the `pg_catalog.pg_largeobject` table schema.

diesel::table! {
    /// `pg_catalog.pg_largeobject` — system catalog holding data pages of large objects.
    /// Large objects are split into chunks stored as separate rows in this table.
    pg_catalog.pg_largeobject (loid, pageno) {
        /// OID of the large object.
        loid -> Oid,
        /// Page number of this page within the large object (zero-based).
        pageno -> Integer,
        /// Actual data stored in this page (max 2KB per page).
        data -> Binary,
    }
}
