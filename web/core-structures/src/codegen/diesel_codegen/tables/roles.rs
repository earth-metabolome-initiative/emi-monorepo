diesel::table! {
    public.roles(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, icon_id -> diesel::sql_types::SmallInt, color_id ->
    diesel::sql_types::SmallInt, id -> diesel::sql_types::SmallInt }
}
