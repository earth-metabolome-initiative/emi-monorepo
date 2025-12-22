diesel::table! {
    procedure_assets(id) { id -> ::rosetta_uuid::diesel_impls::Uuid, procedure ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template -> diesel::sql_types::Integer,
    asset_model_id -> diesel::sql_types::Integer, asset -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, procedure_template_asset_model_id ->
    diesel::sql_types::Integer, ancestor_model -> diesel::sql_types::Integer }
}
