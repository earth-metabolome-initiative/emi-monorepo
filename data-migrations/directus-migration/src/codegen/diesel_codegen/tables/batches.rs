diesel::table! {
    #[sql_name = "Batches"] public.batches(id) { id -> diesel::sql_types::Integer, status
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, uuid_batch ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, batch_id ->
    diesel::sql_types::Text, batch_type -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, comments -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, old_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, short_description -> diesel::sql_types::Text, description
    -> diesel::sql_types::Text }
}
