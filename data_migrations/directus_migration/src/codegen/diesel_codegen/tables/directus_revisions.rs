diesel::table! {
    directus_revisions(id) { id -> diesel::sql_types::Integer, activity ->
    diesel::sql_types::Integer, collection -> diesel::sql_types::Text, item ->
    diesel::sql_types::Text, data -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, delta -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, parent -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, version -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid > }
}
