diesel::table! {
    procedure_template_asset_models(id) { id -> diesel::sql_types::Integer, name ->
    diesel::sql_types::Text, procedure_template -> diesel::sql_types::Integer, based_on
    -> diesel::sql_types::Nullable < diesel::sql_types::Integer >, asset_model ->
    diesel::sql_types::Integer, created_by -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
