//! Submodule for the `pg_catalog.pg_range` table schema.

diesel::table! {
    /// `pg_catalog.pg_range` — system catalog containing information about range types.
    /// Each row defines a range type and its properties.
    pg_catalog.pg_range (rngtypid) {
        /// OID of the range type (references pg_type).
        rngtypid -> Oid,
        /// OID of the element (subtype) type.
        rngsubtype -> Oid,
        /// OID of the corresponding multirange type.
        rngmultitypid -> Oid,
        /// OID of the collation to use for range comparisons, or 0 if none.
        rngcollation -> Oid,
        /// OID of the subtype's operator class.
        rngsubopc -> Oid,
        /// OID of the canonicalization function, or 0 if none.
        rngcanonical -> Oid,
        /// OID of the subtype difference function, or 0 if none.
        rngsubdiff -> Oid,
    }
}
