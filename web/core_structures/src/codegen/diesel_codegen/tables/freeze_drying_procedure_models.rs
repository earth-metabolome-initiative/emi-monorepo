diesel::table! {
    freeze_drying_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float, pascal ->
    diesel::sql_types::Float, seconds -> diesel::sql_types::Float, freeze_dried_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_freeze_dried_with ->
    diesel::sql_types::Integer, source_container -> diesel::sql_types::Integer }
}
