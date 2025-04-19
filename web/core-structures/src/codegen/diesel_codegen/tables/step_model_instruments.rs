diesel::table! {
    public.step_model_instruments(id) { id -> diesel::sql_types::Integer, instrument_id
    -> diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at
    -> rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
