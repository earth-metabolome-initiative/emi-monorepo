diesel::table! {
    ball_mill_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, seconds -> diesel::sql_types::Float, hertz ->
    diesel::sql_types::Float, milled_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
