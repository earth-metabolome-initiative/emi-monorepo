diesel::table! {
    public.nameplate_categories(id) { name -> diesel::sql_types::Text,
    permanence_category_id -> diesel::sql_types::SmallInt, description ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, color_id ->
    diesel::sql_types::SmallInt, id -> diesel::sql_types::SmallInt }
}
