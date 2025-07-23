diesel::table! {
    placing_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, source -> diesel::sql_types::Integer, placed_into ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_placed_into ->
    diesel::sql_types::Integer, quantity -> diesel::sql_types::SmallInt }
}
