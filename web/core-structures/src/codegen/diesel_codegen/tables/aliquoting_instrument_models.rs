diesel::table! {
    public.aliquoting_instrument_models(id) { id -> diesel::sql_types::Integer,
    error_liters -> diesel::sql_types::Float, minimum_measurable_liters ->
    diesel::sql_types::Float, maximum_measurable_liters -> diesel::sql_types::Float,
    created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
