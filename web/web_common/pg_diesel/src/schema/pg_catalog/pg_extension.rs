//! Submodule for the `pg_catalog.pg_extension` table schema.

diesel::table! {
    /// `pg_catalog.pg_extension` — system catalog containing one row per extension installed in the database.
    /// Provides metadata about extension name, owner, schema, version, and any configuration tables.
    pg_catalog.pg_extension (oid) {
        /// OID of the extension.
        oid -> Oid,
        /// Name of the extension.
        extname -> Text,
        /// OID of the user who owns the extension.
        extowner -> Oid,
        /// OID of the schema that contains objects for the extension.
        extnamespace -> Oid,
        /// `true` if the extension is relocatable (can be moved to a different schema); `false` otherwise.
        extrelocatable -> Bool,
        /// Version of the extension.
        extversion -> Text,
        /// Array of OIDs of configuration tables associated with the extension; `NULL` if none.
        extconfig -> Nullable<Array<Oid>>,
        /// Array of conditions for each configuration table; `NULL` if none.
        extcondition -> Nullable<Array<Text>>,
    }
}

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_extension, pg_namespace);

use super::pg_type::pg_type;
diesel::allow_tables_to_appear_in_same_query!(pg_extension, pg_type);

use super::pg_proc::pg_proc;
diesel::allow_tables_to_appear_in_same_query!(pg_extension, pg_proc);

use super::pg_enum::pg_enum;
diesel::allow_tables_to_appear_in_same_query!(pg_extension, pg_enum);
