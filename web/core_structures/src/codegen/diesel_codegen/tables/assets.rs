diesel::table! {
    assets(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, most_concrete_table ->
    diesel::sql_types::Text, name -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, description -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, model -> diesel::sql_types::Integer, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
