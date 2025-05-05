diesel::table! {
    public.units(id) { name -> diesel::sql_types::Text, unit -> diesel::sql_types::Text,
    icon -> diesel::sql_types::Text, color_id -> diesel::sql_types::SmallInt, id ->
    diesel::sql_types::SmallInt }
}
