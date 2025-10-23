//! `applicable_roles` view from `information_schema`.

diesel::table! {
    /// SQL-standard view exposing applicable role information.
    ///
    /// This corresponds to `applicable_roles` in the `information_schema`,
    /// which identifies all roles that are applicable to the current user.
    /// A role is applicable if it has been granted to the current user either
    /// directly or through a chain of role grants.
    ///
    /// See: https://www.postgresql.org/docs/current/infoschema-applicable-roles.html
    information_schema.applicable_roles (grantee, role_name) {
        /// Name of the role to which the role identified in role_name was granted (the grantee).
        grantee -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        /// Name of the role that is applicable to the grantee.
        role_name -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        /// "YES" if the grantee has the option to grant the role to others, "NO" if not.
        is_grantable -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
    }
}
