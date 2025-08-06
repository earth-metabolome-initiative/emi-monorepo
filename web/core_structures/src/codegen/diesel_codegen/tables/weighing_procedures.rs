diesel::table! {
    weighing_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id -> diesel::sql_types::Integer,
    weighted_with -> ::rosetta_uuid::diesel_impls::Uuid, weighted_container_id ->
    ::rosetta_uuid::diesel_impls::Uuid, kilograms -> diesel::sql_types::Float }
}
