diesel::table! {
    public.directus_shares(id) { id -> diesel::sql_types::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, collection ->
    diesel::sql_types::Text, item -> diesel::sql_types::Text, role ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, password ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, date_start ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, date_end ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, times_used ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, max_uses ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
