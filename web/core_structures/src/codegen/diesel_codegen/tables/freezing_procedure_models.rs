diesel::table! {
    freezing_procedure_models(id) { id -> diesel::sql_types::Integer, kelvin ->
    diesel::sql_types::Float, seconds -> diesel::sql_types::Float, frozen_with ->
    diesel::sql_types::Integer, source_container -> diesel::sql_types::Integer }
}
