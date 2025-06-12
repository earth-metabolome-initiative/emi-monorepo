diesel::table! {
    storage_procedure_models(id) { id -> diesel::sql_types::Integer, child_container_id
    -> diesel::sql_types::Integer, parent_container_id -> diesel::sql_types::Integer }
}
