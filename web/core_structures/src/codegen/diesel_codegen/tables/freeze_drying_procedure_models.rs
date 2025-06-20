diesel::table! {
    freeze_drying_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, pascal -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, freeze_dried_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
