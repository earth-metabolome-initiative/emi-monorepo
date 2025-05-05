diesel::table! {
    public.trackable_states(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, color_id ->
    diesel::sql_types::SmallInt, id -> diesel::sql_types::SmallInt }
}
