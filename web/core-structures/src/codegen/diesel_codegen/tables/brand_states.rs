diesel::table! {
    public.brand_states(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, color_id -> diesel::sql_types::SmallInt, icon ->
    diesel::sql_types::Text, id -> diesel::sql_types::SmallInt }
}
