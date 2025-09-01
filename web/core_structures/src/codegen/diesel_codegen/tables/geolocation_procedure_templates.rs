diesel::table! {
    geolocation_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, geolocated_with_model -> diesel::sql_types::Integer,
    procedure_template_geolocated_with_model -> diesel::sql_types::Integer,
    geolocated_asset_model -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, procedure_template_geolocated_asset_model ->
    diesel::sql_types::Integer }
}
