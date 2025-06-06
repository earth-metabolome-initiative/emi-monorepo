diesel::table! {
    directus_access(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, role ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, user ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >, policy ->
    ::rosetta_uuid::diesel_impls::Uuid, sort -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer > }
}
