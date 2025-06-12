diesel::table! {
    aliquoting_procedure_models(id) { id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float, source -> diesel::sql_types::Integer, destination ->
    diesel::sql_types::Integer, aliquoted_with -> diesel::sql_types::Integer }
}
