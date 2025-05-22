diesel::table! {
    public.directus_translations(id) { id -> rosetta_uuid::diesel_impls::Uuid, language
    -> diesel::sql_types::Text, key -> diesel::sql_types::Text, value ->
    diesel::sql_types::Text }
}
