diesel::table! {
    public.directus_webhooks(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, method -> diesel::sql_types::Text, url ->
    diesel::sql_types::Text, status -> diesel::sql_types::Text, data ->
    diesel::sql_types::Bool, actions -> diesel::sql_types::Text, collections ->
    diesel::sql_types::Text, headers -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, was_active_before_deprecation -> diesel::sql_types::Bool,
    migrated_flow -> diesel::sql_types::Nullable < diesel::sql_types::Uuid > }
}
