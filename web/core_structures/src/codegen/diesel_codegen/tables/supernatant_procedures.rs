diesel::table! {
    supernatant_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id -> diesel::sql_types::Integer,
    stratified_source -> ::rosetta_uuid::diesel_impls::Uuid, supernatant_destination ->
    ::rosetta_uuid::diesel_impls::Uuid, transferred_with ->
    ::rosetta_uuid::diesel_impls::Uuid, pipette_tip -> ::rosetta_uuid::diesel_impls::Uuid
    }
}
