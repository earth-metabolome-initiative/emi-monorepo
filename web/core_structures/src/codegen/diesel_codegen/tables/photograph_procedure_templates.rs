diesel::table! {
    photograph_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, photographed_with_model -> diesel::sql_types::Integer,
    procedure_template_photographed_with_model -> diesel::sql_types::Integer,
    photographed_asset_model_id -> diesel::sql_types::Integer,
    procedure_template_photographed_asset_model_id -> diesel::sql_types::Integer,
    photograph_model -> diesel::sql_types::Integer, procedure_template_photograph_model
    -> diesel::sql_types::Integer }
}
