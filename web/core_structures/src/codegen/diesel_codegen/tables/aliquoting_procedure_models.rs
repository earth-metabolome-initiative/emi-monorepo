diesel::table! {
    aliquoting_procedure_models(id) { id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float, error -> diesel::sql_types::Float }
}
