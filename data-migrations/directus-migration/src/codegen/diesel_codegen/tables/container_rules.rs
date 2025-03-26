diesel::table! {
    #[sql_name = "Container_Rules"] public.container_rules(id) { id ->
    diesel::sql_types::Integer, status -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, user_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_created -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, user_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Uuid >, date_updated -> diesel::sql_types::Nullable <
    diesel::sql_types::Timestamptz >, child_container -> diesel::sql_types::Integer,
    parent_container -> diesel::sql_types::Integer, rule_name -> diesel::sql_types::Text
    }
}
