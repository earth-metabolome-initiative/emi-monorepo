diesel::table! {
    disposal_procedure_models(id) { id -> diesel::sql_types::Integer, disposed_id ->
    diesel::sql_types::Integer }
}
