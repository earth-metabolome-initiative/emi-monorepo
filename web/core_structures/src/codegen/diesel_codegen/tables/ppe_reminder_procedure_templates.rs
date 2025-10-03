diesel::table! {
    ppe_reminder_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, ppe_asset_model -> diesel::sql_types::Integer,
    procedure_template_ppe_asset_model -> diesel::sql_types::Integer }
}
