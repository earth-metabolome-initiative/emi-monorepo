//! Submodule for the `pg_catalog.pg_default_acl` table schema.

diesel::table! {
    /// `pg_catalog.pg_default_acl` — system catalog storing default access privileges.
    /// Each row defines default privileges that will be applied to objects created in
    /// the future. These are set using ALTER DEFAULT PRIVILEGES.
    pg_catalog.pg_default_acl (oid) {
        /// OID of this default ACL entry.
        oid -> Oid,
        /// OID of the role that these default privileges are for.
        defaclrole -> Oid,
        /// OID of the namespace these defaults apply to, or 0 for all namespaces.
        defaclnamespace -> Oid,
        /// Type of object this entry is for:
        /// 'r' = relation (table, view),
        /// 'S' = sequence,
        /// 'f' = function,
        /// 'T' = type,
        /// 'n' = schema.
        defaclobjtype -> Text,
        /// Access privileges to be granted by default (ACL entries).
        defaclacl -> Array<Text>,
    }
}
