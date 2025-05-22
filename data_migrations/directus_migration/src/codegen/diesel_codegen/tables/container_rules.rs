diesel::table! {
    #[sql_name = "Container_Rules"] public.container_rules(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    rosetta_uuid::diesel_impls::Uuid >, date_created -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, user_updated ->
    diesel::sql_types::Nullable < rosetta_uuid::diesel_impls::Uuid >, date_updated ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    child_container -> diesel::sql_types::Integer, parent_container ->
    diesel::sql_types::Integer, rule_name -> diesel::sql_types::Text }
}
