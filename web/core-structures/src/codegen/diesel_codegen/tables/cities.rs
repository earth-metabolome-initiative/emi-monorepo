diesel::table! {
    public.cities(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, code -> diesel::sql_types::Text, iso ->
    diesel::sql_types::Text }
}
