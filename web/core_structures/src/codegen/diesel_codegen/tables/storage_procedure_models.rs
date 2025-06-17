diesel::table! {
    storage_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, child_container_id -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_child_container_id -> diesel::sql_types::Integer, parent_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_parent_container_id ->
    diesel::sql_types::Integer }
}
