diesel::table! {
    #[sql_name = "Batch_Types"] batch_types(id) { id -> diesel::sql_types::Integer,
    status -> diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, batch_type ->
    diesel::sql_types::Text, description -> diesel::sql_types::Text }
}
