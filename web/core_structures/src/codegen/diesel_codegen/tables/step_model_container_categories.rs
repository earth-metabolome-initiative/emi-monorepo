diesel::table! {
    step_model_container_categories(id) { id -> diesel::sql_types::Integer, step_model_id
    -> diesel::sql_types::Integer, procedure_model_container_category_id ->
    diesel::sql_types::Integer, expected_kelvin -> diesel::sql_types::Float,
    tolerance_kelvin -> diesel::sql_types::Float, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
