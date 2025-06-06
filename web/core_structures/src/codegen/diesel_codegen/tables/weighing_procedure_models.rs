diesel::table! {
    weighing_procedure_models(id) { id -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float, instrument_id -> diesel::sql_types::Integer }
}
