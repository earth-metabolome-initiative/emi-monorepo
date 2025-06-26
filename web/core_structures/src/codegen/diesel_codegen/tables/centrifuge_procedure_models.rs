diesel::table! {
    centrifuge_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, seconds -> diesel::sql_types::Float, rotation_per_minute
    -> diesel::sql_types::Float, centrifuged_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
