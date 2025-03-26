diesel::table! {
    public.directus_access(id) { id -> diesel::sql_types::Uuid, role ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, user ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, policy ->
    diesel::sql_types::Uuid, sort -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer > }
}
