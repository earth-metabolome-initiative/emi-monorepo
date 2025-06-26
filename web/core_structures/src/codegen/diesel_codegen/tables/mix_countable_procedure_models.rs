diesel::table! {
    mix_countable_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, source -> diesel::sql_types::Integer, destination ->
    diesel::sql_types::Integer, quantity -> diesel::sql_types::SmallInt }
}
