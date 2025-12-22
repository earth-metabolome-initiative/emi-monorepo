diesel::table! {
    geolocation_procedures(procedure) { procedure -> ::rosetta_uuid::diesel_impls::Uuid,
    procedure_template -> diesel::sql_types::Integer, geolocated_asset ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_geolocated_asset_model_id ->
    diesel::sql_types::Integer, procedure_geolocated_asset ->
    ::rosetta_uuid::diesel_impls::Uuid, geolocated_with -> diesel::sql_types::Nullable <
    ::rosetta_uuid::diesel_impls::Uuid >, procedure_geolocated_with ->
    ::rosetta_uuid::diesel_impls::Uuid, procedure_template_geolocated_with_model ->
    diesel::sql_types::Integer, location -> ::postgis_diesel::sql_types::Geography }
}
