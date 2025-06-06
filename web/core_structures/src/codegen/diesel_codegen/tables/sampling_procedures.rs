diesel::table! {
    sampling_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, processable_id ->
    ::rosetta_uuid::diesel_impls::Uuid, trackable_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
