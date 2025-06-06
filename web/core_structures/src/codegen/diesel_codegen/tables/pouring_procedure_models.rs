diesel::table! {
    pouring_procedure_models(id) { id -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float }
}
