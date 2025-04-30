diesel::table! {
    public.instrument_categories(id) { name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, icon -> font_awesome_icons::diesel_impls::FAIcon, id ->
    diesel::sql_types::SmallInt }
}
