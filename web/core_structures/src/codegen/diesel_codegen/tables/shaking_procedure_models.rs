diesel::table! {
    shaking_procedure_models(id) { id -> diesel::sql_types::Integer, seconds ->
    diesel::sql_types::Float }
}
