diesel::table! {
    public.step_model_categories(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, id ->
    diesel::sql_types::SmallInt }
}
