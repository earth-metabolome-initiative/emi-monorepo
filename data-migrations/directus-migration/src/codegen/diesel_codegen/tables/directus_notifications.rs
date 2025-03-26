diesel::table! {
    public.directus_notifications(id) { id -> diesel::sql_types::Integer, timestamp ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, status ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, recipient ->
    diesel::sql_types::Uuid, sender -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, subject -> diesel::sql_types::Text, message ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, collection ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, item ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
