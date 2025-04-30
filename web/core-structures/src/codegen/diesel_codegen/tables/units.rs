diesel::table! {
    public.units(id) { name -> diesel::sql_types::Text, unit -> diesel::sql_types::Text,
    icon -> font_awesome_icons::diesel_impls::FAIcon, color_id ->
    diesel::sql_types::SmallInt, id -> diesel::sql_types::SmallInt }
}
