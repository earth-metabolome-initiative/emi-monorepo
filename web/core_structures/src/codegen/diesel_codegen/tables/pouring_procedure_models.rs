diesel::table! {
    pouring_procedure_models(id) { id -> diesel::sql_types::Integer, measured_with ->
    diesel::sql_types::Integer, source -> diesel::sql_types::Integer, destination ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float }
}
