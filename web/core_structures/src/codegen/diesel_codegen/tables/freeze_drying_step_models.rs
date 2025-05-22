diesel::table! {
    freeze_drying_step_models(id) { id -> diesel::sql_types::Integer, expected_kelvin ->
    diesel::sql_types::Float, expected_pascal -> diesel::sql_types::Float,
    expected_seconds -> diesel::sql_types::Float, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
