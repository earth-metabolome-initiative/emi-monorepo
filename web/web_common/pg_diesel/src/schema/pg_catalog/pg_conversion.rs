//! Submodule for the `pg_catalog.pg_conversion` table schema.

diesel::table! {
    /// `pg_catalog.pg_conversion` — system catalog containing encoding conversion functions.
    /// Each row represents a conversion from one encoding to another, including the function
    /// used to perform the conversion and whether it is the default conversion for those encodings.
    pg_catalog.pg_conversion (oid) {
        /// OID of the conversion.
        oid -> Oid,
        /// Name of the conversion.
        conname -> Text,
        /// OID of the namespace (schema) containing this conversion.
        connamespace -> Oid,
        /// OID of the role that owns the conversion.
        conowner -> Oid,
        /// Source encoding ID (from pg_encoding).
        conforencoding -> Integer,
        /// Destination encoding ID (to pg_encoding).
        contoencoding -> Integer,
        /// OID of the conversion function.
        conproc -> Oid,
        /// `true` if this is the default conversion for this encoding pair.
        condefault -> Bool,
    }
}
