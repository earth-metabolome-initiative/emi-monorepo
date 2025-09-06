diesel::table! {
    aliquoting_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float, aliquoted_from_model
    -> diesel::sql_types::Integer, procedure_template_aliquoted_from_model ->
    diesel::sql_types::Integer, aliquoted_into_model -> diesel::sql_types::Integer,
    procedure_template_aliquoted_into_model -> diesel::sql_types::Integer,
    aliquoted_with_model -> diesel::sql_types::Integer,
    procedure_template_aliquoted_with_model -> diesel::sql_types::Integer,
    pipette_tip_model -> diesel::sql_types::Integer, procedure_template_pipette_tip_model
    -> diesel::sql_types::Integer }
}
