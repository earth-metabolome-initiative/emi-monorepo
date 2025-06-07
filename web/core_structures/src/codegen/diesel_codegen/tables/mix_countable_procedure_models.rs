diesel::table! {
    mix_countable_procedure_models(id) { id -> diesel::sql_types::Integer, source ->
    diesel::sql_types::Integer, destination -> diesel::sql_types::Integer, quantity ->
    diesel::sql_types::SmallInt }
}
