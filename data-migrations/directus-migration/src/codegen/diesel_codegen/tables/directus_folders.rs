diesel::table! {
    public.directus_folders(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Text, parent -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid > }
}
