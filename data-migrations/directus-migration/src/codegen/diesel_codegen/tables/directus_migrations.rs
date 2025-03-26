diesel::table! {
    public.directus_migrations(version) { version -> diesel::sql_types::Text, name ->
    diesel::sql_types::Text, timestamp -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz > }
}
