diesel::table! {
    aliquoting_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id -> diesel::sql_types::Integer,
    aliquoted_with -> ::rosetta_uuid::diesel_impls::Uuid, pipette_tip ->
    ::rosetta_uuid::diesel_impls::Uuid, aliquoted_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
