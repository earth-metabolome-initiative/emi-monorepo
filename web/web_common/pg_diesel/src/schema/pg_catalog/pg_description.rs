//! Submodule for the `pg_catalog.pg_description` table schema.

diesel::table! {
    /// `pg_catalog.pg_description` — table containing comments/descriptions for database objects.
    pg_catalog.pg_description (objoid, classoid, objsubid) {
        /// OID of the object being described.
        objoid -> diesel::sql_types::Oid,
        /// OID of the system catalog the object belongs to.
        classoid -> diesel::sql_types::Oid,
        /// Sub-ID of the object (0 if not applicable).
        objsubid -> Integer,
        /// Textual description/comment for the object.
        description -> Text,
    }
}

use super::pg_namespace::pg_namespace;
diesel::allow_tables_to_appear_in_same_query!(pg_description, pg_namespace);
