diesel::table! {
    public.aliquoting_steps(id) { id -> rosetta_uuid::diesel_impls::Uuid,
    source_processable_id -> rosetta_uuid::diesel_impls::Uuid, destination_processable_id
    -> rosetta_uuid::diesel_impls::Uuid, instrument_id -> diesel::sql_types::Integer,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
