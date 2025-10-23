//! Submodule for the `pg_catalog.pg_language` table schema.

diesel::table! {
    /// `pg_catalog.pg_language` — system catalog containing procedural languages.
    /// Each row defines a language in which functions can be written.
    pg_catalog.pg_language (oid) {
        /// OID of the language.
        oid -> Oid,
        /// Name of the language (e.g., 'sql', 'plpgsql', 'c').
        lanname -> Text,
        /// OID of the role that owns the language.
        lanowner -> Oid,
        /// `true` if this is a procedural language (as opposed to built-in like SQL or C).
        lanispl -> Bool,
        /// `true` if this is a trusted language (can be used by non-superusers).
        lanpltrusted -> Bool,
        /// OID of the call handler function (0 for built-in languages).
        lanplcallfoid -> Oid,
        /// OID of the inline handler function (0 if none).
        laninline -> Oid,
        /// OID of the validator function (0 if none).
        lanvalidator -> Oid,
        /// Access privileges for the language.
        lanacl -> Nullable<Array<Text>>,
    }
}
