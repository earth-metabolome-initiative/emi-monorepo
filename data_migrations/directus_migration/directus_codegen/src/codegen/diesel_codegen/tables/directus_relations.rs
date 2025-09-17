diesel::table! {
    directus_relations(id) { id -> diesel::sql_types::Integer, many_collection ->
    diesel::sql_types::Text, many_field -> diesel::sql_types::Text, one_collection ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, one_field ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, one_collection_field ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, one_allowed_collections ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, junction_field ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, sort_field ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, one_deselect_action ->
    diesel::sql_types::Text }
}
