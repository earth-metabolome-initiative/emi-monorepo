diesel::table! {
    disposal_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, disposed_asset ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_disposed_asset_model -> diesel::sql_types::Integer,
    procedure_disposed_asset -> ::rosetta_uuid::diesel_impls::Uuid }
}
