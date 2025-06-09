diesel::table! {
    weighing_procedures(procedure_id) { procedure_id ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_model_id -> diesel::sql_types::Integer,
    instrument_id -> ::rosetta_uuid::diesel_impls::Uuid, kilograms ->
    diesel::sql_types::Float }
}
