diesel::table! {
    organism_sampling_step_models(id) { id -> diesel::sql_types::Integer, organism_id ->
    ::rosetta_uuid::diesel_impls::Uuid }
}
