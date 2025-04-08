diesel::table! {
    public.directus_folders(id) { id -> rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Text, parent -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid > }
}
