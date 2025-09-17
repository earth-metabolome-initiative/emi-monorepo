diesel::table! {
    directus_shares(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, collection ->
    diesel::sql_types::Text, item -> diesel::sql_types::Text, role ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, password ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, user_created ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, date_created ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    date_start -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC >, date_end ->
    diesel::sql_types::Nullable < rosetta_timestamp::diesel_impls::TimestampUTC >,
    times_used -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, max_uses ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer > }
}
