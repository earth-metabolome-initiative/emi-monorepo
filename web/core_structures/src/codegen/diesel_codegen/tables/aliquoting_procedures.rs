diesel::table! {
    aliquoting_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, source_processable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, destination_processable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, instrument_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
