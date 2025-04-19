diesel::table! {
    public.aliquoting_step_models(id) { id -> diesel::sql_types::Integer,
    step_model_instrument_category_id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
