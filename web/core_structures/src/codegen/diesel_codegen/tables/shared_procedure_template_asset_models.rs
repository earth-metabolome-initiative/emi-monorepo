diesel::table! {
    shared_procedure_template_asset_models(parent, child_id) { parent ->
    diesel::sql_types::Integer, parent_asset_model -> diesel::sql_types::Integer,
    parent_procedure_template -> diesel::sql_types::Integer, child_id ->
    diesel::sql_types::Integer, child_asset_model -> diesel::sql_types::Integer,
    child_procedure_template -> diesel::sql_types::Integer, created_by ->
    diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
