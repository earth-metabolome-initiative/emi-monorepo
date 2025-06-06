diesel::table! {
    processing_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, processable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, instrument_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
