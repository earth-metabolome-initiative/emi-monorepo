diesel::table! {
    public.directus_versions(id) { id -> rosetta_uuid::diesel_impls::Uuid, key ->
    diesel::sql_types::Text, name -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, collection -> diesel::sql_types::Text, item ->
    diesel::sql_types::Text, hash -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_created -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, user_updated -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, delta -> diesel::sql_types::Nullable <
    diesel::sql_types::Json > }
}
