diesel::table! {
    weighing_procedure_models(id) { id -> diesel::sql_types::Integer, instrument_id ->
    diesel::sql_types::Integer, sample_container -> diesel::sql_types::Integer }
}
