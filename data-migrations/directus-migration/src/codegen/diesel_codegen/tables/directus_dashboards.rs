diesel::table! {
    public.directus_dashboards(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, color ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
