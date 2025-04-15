diesel::table! {
    #[sql_name = "Universities"] public.universities(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, user_updated ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    uuid_university -> diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >,
    university_name -> diesel::sql_types::Text, country -> diesel::sql_types::Text,
    alpha_two -> diesel::sql_types::Text, web_pages -> diesel::sql_types::Text, state ->
    diesel::sql_types::Text, domains -> diesel::sql_types::Text }
}
