diesel::table! {
    #[sql_name = "Batches"] public.batches(id) { id -> diesel::sql_types::Integer, status
    -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, uuid_batch ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, batch_id ->
    diesel::sql_types::Text, batch_type -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, comments -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, old_id -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, short_description -> diesel::sql_types::Text, description
    -> diesel::sql_types::Text }
}
