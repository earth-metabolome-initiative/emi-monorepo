diesel::table! {
    public.directus_extensions(id) { enabled -> diesel::sql_types::Bool, id ->
    diesel::sql_types::Uuid, folder -> diesel::sql_types::Text, source ->
    diesel::sql_types::Text, bundle -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid > }
}
