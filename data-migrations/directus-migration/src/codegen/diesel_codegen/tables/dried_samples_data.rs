diesel::table! {
    #[sql_name = "Dried_Samples_Data"] public.dried_samples_data(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Text, user_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_created ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, user_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, date_updated ->
    diesel::sql_types::Nullable < diesel::sql_types::Timestamptz >, uuid_dried_sample ->
    diesel::sql_types::Nullable < diesel::sql_types::Uuid >, sample_container ->
    diesel::sql_types::Integer, parent_container -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, batch -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, field_data -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer > }
}
