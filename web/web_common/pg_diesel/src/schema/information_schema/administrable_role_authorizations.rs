//! `administrable_role_authorizations` view from `information_schema`.

diesel::table! {
    /// SQL-standard view exposing role authorization information.
    ///
    /// This corresponds to `administrable_role_authorizations` in the `information_schema`,
    /// which identifies all role authorizations that are administrable by the current user.
    /// A role authorization represents the granting of a role to a user or another role.
    ///
    /// See: https://www.postgresql.org/docs/current/infoschema-administrable-role-authorizations.html
    information_schema.administrable_role_authorizations (grantee, role_name) {
        /// Name of the role that was granted the privilege (the grantee).
        grantee -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        /// Name of the role that was granted to the grantee.
        role_name -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
        /// "YES" if the grantee has the option to grant the role to others, "NO" if not.
        is_grantable -> diesel::sql_types::Nullable<diesel::sql_types::Text>,
    }
}
