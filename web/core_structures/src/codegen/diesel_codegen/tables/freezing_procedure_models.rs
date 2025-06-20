diesel::table! {
    freezing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, seconds -> diesel::sql_types::Nullable <
    diesel::sql_types::Float >, frozen_with -> ::rosetta_uuid::diesel_impls::Uuid }
}
