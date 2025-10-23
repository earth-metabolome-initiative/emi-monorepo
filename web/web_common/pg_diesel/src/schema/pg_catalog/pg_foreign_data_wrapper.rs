//! Submodule for the `pg_catalog.pg_foreign_data_wrapper` table schema.

diesel::table! {
    /// `pg_catalog.pg_foreign_data_wrapper` — system catalog containing foreign data wrappers.
    /// Each row describes a foreign data wrapper, which allows access to data in external
    /// data sources.
    pg_catalog.pg_foreign_data_wrapper (oid) {
        /// OID of the foreign data wrapper.
        oid -> Oid,
        /// Name of the foreign data wrapper.
        fdwname -> Text,
        /// OID of the role that owns the foreign data wrapper.
        fdwowner -> Oid,
        /// OID of the handler function (0 if none).
        fdwhandler -> Oid,
        /// OID of the validator function (0 if none).
        fdwvalidator -> Oid,
        /// Access privileges for the foreign data wrapper.
        fdwacl -> Nullable<Array<Text>>,
        /// Foreign data wrapper-specific options, stored as "name=value" strings.
        fdwoptions -> Nullable<Array<Text>>,
    }
}
