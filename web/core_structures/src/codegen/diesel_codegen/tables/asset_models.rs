diesel::table! {
    asset_models(id) { id -> diesel::sql_types::Integer, most_concrete_table ->
    diesel::sql_types::Text, name -> diesel::sql_types::Text, description ->
    diesel::sql_types::Text, parent_model -> diesel::sql_types::Nullable <
    diesel::sql_types::Integer >, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC, updated_by ->
    diesel::sql_types::Integer, updated_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
