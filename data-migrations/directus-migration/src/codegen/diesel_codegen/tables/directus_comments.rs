diesel::table! {
    public.directus_comments(id) { id -> diesel::sql_types::Uuid, collection ->
    diesel::sql_types::Text, item -> diesel::sql_types::Text, comment ->
    diesel::sql_types::Text, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid > }
}
