diesel::table! {
    directus_roles(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, name ->
    diesel::sql_types::Text, icon -> diesel::sql_types::Text, description ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, parent ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid > }
}
