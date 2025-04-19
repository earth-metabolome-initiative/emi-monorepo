diesel::table! {
    public.fractioning_steps(id) { id -> rosetta_uuid::diesel_impls::Uuid,
    source_processable_id -> rosetta_uuid::diesel_impls::Uuid, destination_processable_id
    -> rosetta_uuid::diesel_impls::Uuid, fractioning_step_model_id ->
    diesel::sql_types::Integer, instrument_id -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
