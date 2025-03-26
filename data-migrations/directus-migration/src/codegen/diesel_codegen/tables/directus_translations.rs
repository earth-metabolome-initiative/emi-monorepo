diesel::table! {
    public.directus_translations(id) { id -> diesel::sql_types::Uuid, language ->
    diesel::sql_types::Text, key -> diesel::sql_types::Text, value ->
    diesel::sql_types::Text }
}
