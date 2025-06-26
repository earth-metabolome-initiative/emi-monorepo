diesel::table! {
    capping_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, container_id -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_container_id -> diesel::sql_types::Integer, capped_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_capped_with ->
    diesel::sql_types::Integer }
}
