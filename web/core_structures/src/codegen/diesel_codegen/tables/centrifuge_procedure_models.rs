diesel::table! {
    centrifuge_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, seconds -> diesel::sql_types::Float, rotation_per_minute
    -> diesel::sql_types::Float, centrifuged_with -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_centrifuged_with -> diesel::sql_types::Integer, container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_container_id ->
    diesel::sql_types::Integer }
}
