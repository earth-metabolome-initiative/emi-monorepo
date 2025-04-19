diesel::table! {
    public.storage_containers(id) { id -> rosetta_uuid::diesel_impls::Uuid,
    container_model_id -> diesel::sql_types::Integer, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
