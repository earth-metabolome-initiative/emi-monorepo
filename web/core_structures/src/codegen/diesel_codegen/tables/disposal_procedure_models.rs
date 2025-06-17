diesel::table! {
    disposal_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, disposed_id -> diesel::sql_types::Integer }
}
