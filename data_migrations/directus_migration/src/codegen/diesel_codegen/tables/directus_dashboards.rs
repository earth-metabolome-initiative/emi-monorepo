diesel::table! {
    public.directus_dashboards(id) { id -> rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_created -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    color -> diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
