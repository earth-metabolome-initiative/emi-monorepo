diesel::table! {
    tagging_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, tagged_asset ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_tagged_asset_model -> diesel::sql_types::Integer,
    procedure_tagged_asset -> ::rosetta_uuid::diesel_impls::Uuid, tag_asset ->
    diesel::sql_types::Nullable < ::rosetta_uuid::diesel_impls::Uuid >,
    procedure_template_tag_asset_model -> diesel::sql_types::Integer, procedure_tag_asset
    -> ::rosetta_uuid::diesel_impls::Uuid, geolocated_with -> diesel::sql_types::Nullable
    < ::rosetta_uuid::diesel_impls::Uuid >, procedure_geolocated_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_geolocated_with_model ->
    diesel::sql_types::Integer, location -> ::postgis_diesel::sql_types::Geography }
}
