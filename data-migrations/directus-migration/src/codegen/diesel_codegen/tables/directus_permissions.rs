diesel::table! {
    public.directus_permissions(id) { id -> diesel::sql_types::Integer, collection ->
    diesel::sql_types::Text, action -> diesel::sql_types::Text, permissions ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, validation ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, presets ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, fields ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, policy ->
    diesel::sql_types::Uuid }
}
