diesel::table! {
    disposal_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, disposed_asset_model_id -> diesel::sql_types::Integer,
    procedure_template_disposed_asset_model_id -> diesel::sql_types::Integer }
}
