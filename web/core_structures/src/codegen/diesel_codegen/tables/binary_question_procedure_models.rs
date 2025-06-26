diesel::table! {
    binary_question_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, trackable_id -> diesel::sql_types::Integer }
}
