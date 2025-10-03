diesel::table! {
    tagging_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, geolocated_with_model -> diesel::sql_types::Integer,
    procedure_template_geolocated_with_model -> diesel::sql_types::Integer,
    tagged_asset_model -> diesel::sql_types::Integer,
    procedure_template_tagged_asset_model -> diesel::sql_types::Integer, tag_asset_model
    -> diesel::sql_types::Integer, procedure_template_tag_asset_model ->
    diesel::sql_types::Integer }
}
