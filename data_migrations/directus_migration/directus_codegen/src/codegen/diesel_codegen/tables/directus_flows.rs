diesel::table! {
    directus_flows(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, color -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, description -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, status -> diesel::sql_types::Text, trigger ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, accountability ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, options ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, operation ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    user_created -> diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid > }
}
