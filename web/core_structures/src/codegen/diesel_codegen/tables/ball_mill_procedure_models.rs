diesel::table! {
    ball_mill_procedure_models(id) { id -> diesel::sql_types::Integer, seconds ->
    diesel::sql_types::Float, hertz -> diesel::sql_types::Float, milled_with ->
    diesel::sql_types::Integer, container_id -> diesel::sql_types::Integer }
}
