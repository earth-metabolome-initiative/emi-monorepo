diesel::table! {
    mount_tip_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, pipette -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_pipette -> diesel::sql_types::Integer, pipette_tip ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_pipette_tip ->
    diesel::sql_types::Integer }
}
