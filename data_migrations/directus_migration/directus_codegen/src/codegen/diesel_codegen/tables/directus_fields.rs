diesel::table! {
    directus_fields(id) { id -> diesel::sql_types::Integer, collection ->
    diesel::sql_types::Text, field -> diesel::sql_types::Text, special ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, interface ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, options ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, display ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, display_options ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, readonly ->
    diesel::sql_types::Bool, hidden -> diesel::sql_types::Bool, sort ->
    diesel::sql_types::Nullable < diesel::sql_types::Integer >, width ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, translations ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, note ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, conditions ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, required ->
    diesel::sql_types::Nullable < diesel::sql_types::Bool >, group ->
    diesel::sql_types::Nullable < diesel::sql_types::Text >, validation ->
    diesel::sql_types::Nullable < diesel::sql_types::Json >, validation_message ->
    diesel::sql_types::Nullable < diesel::sql_types::Text > }
}
