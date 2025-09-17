diesel::table! {
    disposal_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, disposed_asset_model -> diesel::sql_types::Integer,
    procedure_template_disposed_asset_model -> diesel::sql_types::Integer }
}
