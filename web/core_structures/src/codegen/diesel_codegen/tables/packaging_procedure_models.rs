diesel::table! {
    packaging_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, packaged_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_packaged_with -> diesel::sql_types::Integer, procedure_sample_id ->
    diesel::sql_types::Integer }
}
