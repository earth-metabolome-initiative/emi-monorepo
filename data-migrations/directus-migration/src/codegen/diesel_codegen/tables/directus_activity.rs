diesel::table! {
    public.directus_activity(id) { id -> diesel::sql_types::Integer, action ->
    diesel::sql_types::Text, user -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, timestamp -> diesel::sql_types::Timestamptz, ip ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_agent ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, collection ->
    diesel::sql_types::Text, item -> diesel::sql_types::Text, comment ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, origin ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
