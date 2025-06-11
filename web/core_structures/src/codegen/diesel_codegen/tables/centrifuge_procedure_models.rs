diesel::table! {
    centrifuge_procedure_models(id) { id -> diesel::sql_types::Integer, seconds ->
    diesel::sql_types::Float, rotation_per_minute -> diesel::sql_types::Float,
    centrifuged_with -> diesel::sql_types::Integer, container_id ->
    diesel::sql_types::Integer }
}
