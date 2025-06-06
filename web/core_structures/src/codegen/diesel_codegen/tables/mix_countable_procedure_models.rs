diesel::table! {
    mix_countable_procedure_models(id) { id -> diesel::sql_types::Integer, quantity ->
    diesel::sql_types::SmallInt }
}
