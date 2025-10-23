//! Submodule for the `pg_catalog.pg_init_privs` table schema.

diesel::table! {
    /// `pg_catalog.pg_init_privs` — system catalog recording the initial privileges of objects.
    /// This catalog stores the privileges that objects had when they were created, which is
    /// useful for extensions and pg_dump to restore proper default privileges.
    pg_catalog.pg_init_privs (objoid, classoid, objsubid) {
        /// OID of the specific object.
        objoid -> Oid,
        /// OID of the system catalog containing the object (e.g., pg_class for tables).
        classoid -> Oid,
        /// Object sub-ID (e.g., column number for a column, 0 for the whole object).
        objsubid -> Integer,
        /// Type of privilege: 'i' = initial privileges, 'e' = extension privileges.
        privtype -> Text,
        /// Access privileges as originally assigned (ACL array).
        initprivs -> Array<Text>,
    }
}
