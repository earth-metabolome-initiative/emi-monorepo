//! Submodule for the `information_schema.domain_constraints` view schema.

diesel::table! {
    /// `information_schema.domain_constraints` — view containing one row per constraint
    /// defined on a domain. Provides metadata about domain-level constraints,
    /// including deferrability and initial enforcement timing.
    information_schema.domain_constraints (constraint_catalog, constraint_schema, constraint_name) {
        /// Catalog (database) containing the domain constraint.
        constraint_catalog -> Text,
        /// Schema containing the domain constraint.
        constraint_schema -> Text,
        /// Name of the domain constraint.
        constraint_name -> Text,
        /// Catalog (database) containing the domain; `NULL` if not applicable.
        domain_catalog -> Nullable<Text>,
        /// Schema containing the domain; `NULL` if not applicable.
        domain_schema -> Nullable<Text>,
        /// Name of the domain; `NULL` if not applicable.
        domain_name -> Nullable<Text>,
        /// "YES" if the constraint is deferrable; "NO" otherwise.
        is_deferrable -> Text,
        /// "YES" if the constraint is initially deferred; "NO" otherwise.
        initially_deferred -> Text,
    }
}
