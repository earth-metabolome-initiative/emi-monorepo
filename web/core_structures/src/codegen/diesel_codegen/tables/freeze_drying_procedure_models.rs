diesel::table! {
    freeze_drying_procedure_models(id) { id -> diesel::sql_types::Integer, kelvin ->
    diesel::sql_types::Float, pascal -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, freeze_dried_with -> diesel::sql_types::Integer }
}
