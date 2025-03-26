diesel::table! {
    public.directus_flows(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, color -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, description -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, status -> diesel::sql_types::Text, trigger ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, accountability ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, options ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, operation ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid > }
}
