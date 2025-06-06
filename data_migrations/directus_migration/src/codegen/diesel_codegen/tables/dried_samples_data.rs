diesel::table! {
    #[sql_name = "Dried_Samples_Data"] dried_samples_data(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_updated -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    date_updated -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, uuid_dried_sample ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, sample_container
    -> diesel::sql_types::Integer, parent_container -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, batch -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, field_data -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer > }
}
