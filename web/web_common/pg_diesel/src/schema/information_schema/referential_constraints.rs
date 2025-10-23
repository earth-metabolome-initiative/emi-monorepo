//! Submodule for the `information_schema.referential_constraints` view schema.

diesel::table! {
    /// `information_schema.referential_constraints` — view containing one row per foreign key constraint.
    /// Provides metadata linking the foreign key to its referenced unique or primary key,
    /// along with referential actions and match options.
    information_schema.referential_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the foreign key constraint.
        constraint_catalog -> Text,
        /// Schema containing the foreign key constraint.
        constraint_schema -> Text,
        /// Name of the foreign key constraint.
        constraint_name -> Text,
        /// Catalog of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_catalog -> Nullable<Text>,
        /// Schema of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_schema -> Nullable<Text>,
        /// Name of the referenced unique or primary key constraint; `NULL` if not applicable.
        unique_constraint_name -> Nullable<Text>,
        /// Match option for the foreign key: "FULL", "PARTIAL", or "SIMPLE".
        match_option -> Text,
        /// Action to perform on update of the referenced key: "NO ACTION", "CASCADE", "SET NULL", "SET DEFAULT", or "RESTRICT".
        update_rule -> Text,
        /// Action to perform on delete of the referenced key: "NO ACTION", "CASCADE", "SET NULL", "SET DEFAULT", or "RESTRICT".
        delete_rule -> Text,
    }
}

use super::table_constraints::table_constraints;
diesel::allow_tables_to_appear_in_same_query!(referential_constraints, table_constraints);
