diesel::table! {
    supernatant_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float, stratified_source ->
    diesel::sql_types::Integer, supernatant_destination -> diesel::sql_types::Integer,
    transferred_with -> diesel::sql_types::Integer }
}
