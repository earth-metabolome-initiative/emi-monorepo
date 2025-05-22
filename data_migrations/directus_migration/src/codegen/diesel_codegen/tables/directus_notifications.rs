diesel::table! {
    public.directus_notifications(id) { id -> diesel::sql_types::Integer, timestamp ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >, status
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, recipient ->
    rosetta_uuid::diesel_impls::Uuid, sender -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, subject -> diesel::sql_types::Text, message ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, collection ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, item ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
