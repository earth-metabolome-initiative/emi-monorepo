//! Submodule for the `pg_catalog.pg_collation` table schema.

diesel::table! {
    /// `pg_catalog.pg_collation` — catalog of collations.
    /// Contains information about collations (sorting and character classification rules).
    pg_catalog.pg_collation (oid) {
        /// OID of the collation (primary key).
        oid -> diesel::sql_types::Oid,
        /// Name of the collation.
        collname -> Text,
        /// OID of the namespace containing this collation.
        collnamespace -> diesel::sql_types::Oid,
        /// OID of the owner of this collation.
        collowner -> diesel::sql_types::Oid,
        /// Provider of the collation (c=libc, i=icu, d=database default).
        collprovider -> Text,
        /// Whether the collation is deterministic.
        collisdeterministic -> Bool,
        /// Encoding for which this collation is applicable (-1 for any).
        collencoding -> Integer,
        /// LC_COLLATE setting for this collation.
        collcollate -> Nullable<Text>,
        /// LC_CTYPE setting for this collation.
        collctype -> Nullable<Text>,
        /// Locale name for ICU collations.
        colllocale -> Nullable<Text>,
        /// ICU rules for this collation.
        collicurules -> Nullable<Text>,
        /// Provider-specific version string.
        collversion -> Nullable<Text>,
    }
}
