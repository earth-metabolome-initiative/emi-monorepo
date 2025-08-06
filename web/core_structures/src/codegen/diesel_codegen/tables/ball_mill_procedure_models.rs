diesel::table! {
    ball_mill_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, hertz -> diesel::sql_types::Float, milled_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_milled_with ->
    diesel::sql_types::Integer, milled_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_milled_container_id ->
    diesel::sql_types::Integer }
}
