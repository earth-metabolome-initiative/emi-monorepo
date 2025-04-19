diesel::table! {
    public.trackables(id) { id -> rosetta_uuid::diesel_impls::Uuid, container_model_id ->
    diesel::sql_types::Integer, project_id -> diesel::sql_types::Integer,
    trackable_state_id -> diesel::sql_types::SmallInt, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
