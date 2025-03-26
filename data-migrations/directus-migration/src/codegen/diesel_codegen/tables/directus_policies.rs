diesel::table! {
    public.directus_policies(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, description ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, ip_access ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, enforce_tfa ->
    diesel::sql_types::Bool, admin_access -> diesel::sql_types::Bool, app_access ->
    diesel::sql_types::Bool }
}
