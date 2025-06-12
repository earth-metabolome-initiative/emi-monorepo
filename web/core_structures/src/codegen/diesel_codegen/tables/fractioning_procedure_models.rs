diesel::table! {
    fractioning_procedure_models(id) { id -> diesel::sql_types::Integer, kilograms ->
    diesel::sql_types::Float, tolerance_percentage -> diesel::sql_types::Float,
    weighed_with -> diesel::sql_types::Integer, source -> diesel::sql_types::Integer,
    destination -> diesel::sql_types::Integer }
}
