diesel::table! {
    trackable_locations(id) { id -> rosetta_uuid::diesel_impls::Uuid, trackable_id ->
    rosetta_uuid::diesel_impls::Uuid, storage_container_id -> diesel::sql_types::Nullable
    < rosetta_uuid::diesel_impls::Uuid >, geolocation ->
    postgis_diesel::sql_types::Geography, inferred -> diesel::sql_types::Bool, created_at
    -> rosetta_timestamp::diesel_impls::TimestampUTC, created_by ->
    diesel::sql_types::Integer }
}
