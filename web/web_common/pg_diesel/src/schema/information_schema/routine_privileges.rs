//! Submodule for the `information_schema.routine_privileges` view schema.

diesel::table! {
    /// `information_schema.routine_privileges` — view containing one row for each
    /// privilege granted to a routine in the current database.
    information_schema.routine_privileges (grantor, grantee, specific_catalog, specific_schema, specific_name, routine_catalog, routine_schema, routine_name, privilege_type) {
        /// User who granted the privilege.
        grantor -> Nullable<Text>,
        /// User who received the privilege.
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
