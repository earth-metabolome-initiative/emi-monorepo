//! Submodule for the `pg_catalog.pg_foreign_table` table schema.

diesel::table! {
    /// `pg_catalog.pg_foreign_table` — system catalog containing additional information
    /// about foreign tables. Each row supplements an entry in pg_class for a foreign table.
    pg_catalog.pg_foreign_table (ftrelid) {
        /// OID of the pg_class entry for this foreign table.
        ftrelid -> Oid,
        /// OID of the foreign server for this foreign table.
        ftserver -> Oid,
        /// Foreign table options, stored as "name=value" strings.
        ftoptions -> Nullable<Array<Text>>,
    }
}
