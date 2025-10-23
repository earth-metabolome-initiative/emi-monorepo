//! Submodule for the `information_schema.enabled_roles` view schema.

diesel::table! {
    /// `information_schema.enabled_roles` — view containing one row for each role
    /// that is enabled for the current user.
    information_schema.enabled_roles (role_name) {
        /// Name of the enabled role.
        role_name -> Nullable<Text>,
    }
}
