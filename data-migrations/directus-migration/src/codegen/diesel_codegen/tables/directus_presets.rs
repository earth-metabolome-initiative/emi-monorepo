diesel::table! {
    public.directus_presets(id) { id -> diesel::sql_types::Integer, bookmark ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, role ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, collection ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, search ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, layout ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, layout_query ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, layout_options ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, refresh_interval ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, filter ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, icon ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, color ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
