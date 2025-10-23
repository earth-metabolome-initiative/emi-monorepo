//! Submodule for the `information_schema.role_routine_grants` view schema.

diesel::table! {
    /// `information_schema.role_routine_grants` — view containing one row for each
    /// routine privilege granted to or by a role in the current database.
    information_schema.role_routine_grants (grantor, grantee, specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name, privilege_type) {
        /// Role that granted the privilege.
        grantor -> Nullable<Text>,
        /// Role that received the privilege.
        grantee -> Nullable<Text>,
        /// Catalog (database) containing the specific routine.
        specific_catalog -> Nullable<Text>,
        /// Schema containing the specific routine.
        specific_schema -> Nullable<Text>,
        /// Specific name of the routine.
        specific_name -> Nullable<Text>,
        /// Catalog (database) containing the routine.
        routine_catalog -> Nullable<Text>,
        /// Schema containing the routine.
        routine_schema -> Nullable<Text>,
        /// Name of the routine.
        routine_name -> Nullable<Text>,
        /// Type of privilege granted.
        privilege_type -> Nullable<Text>,
        /// Whether the privilege is grantable.
        is_grantable -> Nullable<Text>,
    }
}
