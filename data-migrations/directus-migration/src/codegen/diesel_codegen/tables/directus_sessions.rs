diesel::table! {
    public.directus_sessions(token) { token -> diesel::sql_types::Text, user ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, expires ->
    diesel::sql_types::Timestamptz, ip -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_agent -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, share -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, origin -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, next_token -> diesel::sql_types::Nullable <
    diesel::sql_types::Text > }
}
