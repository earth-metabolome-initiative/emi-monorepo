diesel::table! {
    procedure_assets(procedure, asset_model) { procedure ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template -> diesel::sql_types::Integer,
    asset_model -> diesel::sql_types::Integer, asset -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, procedure_template_asset_model ->
    diesel::sql_types::Integer, ancestor_model -> diesel::sql_types::Integer, created_by
    -> diesel::sql_types::Integer, created_at ->
    rosetta_timestamp::diesel_impls::TimestampUTC }
}
