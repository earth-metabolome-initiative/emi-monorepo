diesel::table! {
    storage_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, stored_into_model ->
    diesel::sql_types::Integer, procedure_template_stored_into_model ->
    diesel::sql_types::Integer, stored_asset_model -> diesel::sql_types::Integer,
    procedure_template_stored_asset_model -> diesel::sql_types::Integer }
}
