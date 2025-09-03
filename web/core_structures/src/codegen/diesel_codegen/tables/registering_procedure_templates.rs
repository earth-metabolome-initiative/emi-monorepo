diesel::table! {
    registering_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, registered_asset_model -> diesel::sql_types::Integer,
    procedure_template_registered_asset_model -> diesel::sql_types::Integer }
}
