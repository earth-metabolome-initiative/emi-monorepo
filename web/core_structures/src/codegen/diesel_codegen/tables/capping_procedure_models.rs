diesel::table! {
    capping_procedure_models(id) { id -> diesel::sql_types::Integer, container_id ->
    diesel::sql_types::Integer, capped_with -> diesel::sql_types::Integer }
}
