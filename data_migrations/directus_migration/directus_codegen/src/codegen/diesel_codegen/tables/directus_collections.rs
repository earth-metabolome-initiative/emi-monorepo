diesel::table! {
    directus_collections(collection) { collection -> diesel::sql_types::Text, icon ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, display_template ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, hidden ->
    diesel::sql_types::Bool, singleton -> diesel::sql_types::Bool, translations ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, archive_field ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, archive_app_filter ->
    diesel::sql_types::Bool, archive_value -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, unarchive_value -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, sort_field -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, accountability -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, color -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, item_duplication_fields -> diesel::sql_types::Nullable <
    diesel::sql_types::Json >, sort -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, group -> diesel::sql_types::Nullable <
    diesel::sql_types::Text >, collapse -> diesel::sql_types::Text, preview_url ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, versioning ->
    diesel::sql_types::Bool }
}
