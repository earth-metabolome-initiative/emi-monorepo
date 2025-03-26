diesel::table! {
    public.directus_roles(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, description ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, parent ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid > }
}
