diesel::table! {
    centrifuge_procedure_models(id) { id -> diesel::sql_types::Integer, seconds ->
    diesel::sql_types::Float, rotation_per_minute -> diesel::sql_types::Float }
}
