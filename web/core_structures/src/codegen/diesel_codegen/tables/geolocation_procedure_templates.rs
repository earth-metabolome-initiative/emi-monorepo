diesel::table! {
    geolocation_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, geolocated_with_model -> diesel::sql_types::Integer,
    procedure_template_geolocated_with_model -> diesel::sql_types::Integer,
    geolocated_asset_model_id -> diesel::sql_types::Integer,
    procedure_template_geolocated_asset_model_id -> diesel::sql_types::Integer }
}
