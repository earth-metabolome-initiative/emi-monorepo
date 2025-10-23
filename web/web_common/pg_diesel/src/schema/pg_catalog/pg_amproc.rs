//! Submodule for the `pg_catalog.pg_amproc` table schema.

diesel::table! {
    /// `pg_catalog.pg_amproc` — catalog of access method support procedures.
    /// Contains information about support procedures used by access method operator families.
    pg_catalog.pg_amproc (oid) {
        /// OID of this entry (primary key).
        oid -> diesel::sql_types::Oid,
        /// OID of operator family this procedure belongs to.
        amprocfamily -> diesel::sql_types::Oid,
        /// Left-hand input data type, or zero if not type-specific.
        amproclefttype -> diesel::sql_types::Oid,
        /// Right-hand input data type, or zero if not type-specific.
        amprocrighttype -> diesel::sql_types::Oid,
        /// Support procedure number.
        amprocnum -> SmallInt,
        /// OID of the procedure.
        amproc -> diesel::sql_types::Oid,
    }
}
