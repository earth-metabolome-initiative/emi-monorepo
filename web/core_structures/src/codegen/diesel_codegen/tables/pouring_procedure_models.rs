diesel::table! {
    pouring_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, measured_with -> diesel::sql_types::Integer, source ->
    diesel::sql_types::Integer, poured_into -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_poured_into -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float }
}
