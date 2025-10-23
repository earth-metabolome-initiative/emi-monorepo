//! Submodule for the `pg_enum` table schema.

diesel::table! {
    /// `pg_enum` — system catalog containing one row for each label of an enum type.
    /// Stores the textual labels and sort ordering of enum members associated with a user-defined enum type.
    pg_catalog.pg_enum (oid) {
        /// OID of the enum value.
        oid -> Oid,
        /// OID of the enum type that this label belongs to (references `pg_type.oid`).
        enumtypid -> Oid,
        /// Sort order of this enum label within the enum type.
        /// Lower values sort before higher values.
        /// Used internally for ordering comparisons.
        enumsortorder -> Float,
        /// The textual label of the enum value.
        enumlabel -> Text,
    }
}

use super::pg_type::pg_type;
diesel::allow_tables_to_appear_in_same_query!(pg_enum, pg_type);

use super::pg_depend::pg_depend;
diesel::allow_tables_to_appear_in_same_query!(pg_enum, pg_depend);
