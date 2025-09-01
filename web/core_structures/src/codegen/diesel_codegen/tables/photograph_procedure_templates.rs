diesel::table! {
    photograph_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, photographed_with_model -> diesel::sql_types::Integer,
    procedure_template_photographed_with_model -> diesel::sql_types::Integer,
    photographed_asset_model -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, procedure_template_photographed_asset_model ->
    diesel::sql_types::Integer }
}
