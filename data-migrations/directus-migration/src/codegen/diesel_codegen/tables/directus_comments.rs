diesel::table! {
    public.directus_comments(id) { id -> rosetta_uuid::diesel_impls::Uuid, collection ->
    diesel::sql_types::Text, item -> diesel::sql_types::Text, comment ->
    diesel::sql_types::Text, date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, date_updated ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_created -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    user_updated -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid > }
}
