//! Schema for pg_catalog.pg_transform table.

diesel::table! {
    use diesel::sql_types::*;

    /// Type transformations
    ///
    /// The catalog `pg_transform` stores information about transforms, which are
    /// mechanisms for converting data between SQL types and procedural language types.
    pg_catalog.pg_transform (oid) {
        /// Row identifier
        oid -> Oid,
        /// OID of the data type this transform is for
        trftype -> Oid,
        /// OID of the language this transform is for
        trflang -> Oid,
        /// OID of the function to transform from SQL to the language
        trffromsql -> Oid,
        /// OID of the function to transform from the language to SQL
        trftosql -> Oid,
    }
}
