diesel::table! {
    public.permanence_categories(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, icon -> font_awesome_icons::diesel_impls::FAIcon, color_id
    -> diesel::sql_types::SmallInt, id -> diesel::sql_types::SmallInt }
}
