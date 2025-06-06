diesel::table! {
    directus_migrations(version) { version -> diesel::sql_types::Text, name ->
    diesel::sql_types::Text, timestamp -> diesel::sql_types::Nullable <
    rosetta_timestamp::diesel_impls::TimestampUTC > }
}
