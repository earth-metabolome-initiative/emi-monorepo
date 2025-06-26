diesel::table! {
    mix_solid_procedure_models(procedure_model_id) { procedure_model_id ->
    diesel::sql_types::Integer, measured_with -> diesel::sql_types::Integer, source ->
    diesel::sql_types::Integer, destination -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float }
}
