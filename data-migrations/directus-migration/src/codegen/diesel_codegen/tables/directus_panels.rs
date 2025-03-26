diesel::table! {
    public.directus_panels(id) { id -> diesel::sql_types::Uuid, dashboard ->
    diesel::sql_types::Uuid, name -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, icon -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, color -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, show_header -> diesel::sql_types::Bool, note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, #[sql_name = "type"] r#type
    -> diesel::sql_types::Text, position_x -> diesel::sql_types::Integer, position_y ->
    diesel::sql_types::Integer, width -> diesel::sql_types::Integer, height ->
    diesel::sql_types::Integer, options -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid > }
}
