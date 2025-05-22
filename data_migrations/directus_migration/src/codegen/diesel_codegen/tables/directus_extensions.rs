diesel::table! {
    public.directus_extensions(id) { enabled -> diesel::sql_types::Bool, id ->
    rosetta_uuid::diesel_impls::Uuid, folder -> diesel::sql_types::Text, source ->
    diesel::sql_types::Text, bundle -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid > }
}
