diesel::table! {
    weighing_instrument_models(id) { id -> diesel::sql_types::Integer, error_kilograms ->
    diesel::sql_types::Float, minimum_measurable_kilograms -> diesel::sql_types::Float,
    maximum_measurable_kilograms -> diesel::sql_types::Float, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
