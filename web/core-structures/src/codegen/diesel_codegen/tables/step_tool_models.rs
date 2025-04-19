diesel::table! {
    public.step_tool_models(id) { id -> rosetta_uuid::diesel_impls::Uuid, step_id ->
    rosetta_uuid::diesel_impls::Uuid, tool_model_id -> diesel::sql_types::Integer,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
