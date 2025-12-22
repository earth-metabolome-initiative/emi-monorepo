diesel::table! {
    supernatant_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, liters -> diesel::sql_types::Float,
    stratified_source_model -> diesel::sql_types::Integer,
    procedure_template_stratified_source_model -> diesel::sql_types::Integer,
    supernatant_destination_model -> diesel::sql_types::Integer,
    procedure_template_supernatant_destination_model -> diesel::sql_types::Integer,
    transferred_with_model -> diesel::sql_types::Integer,
    procedure_template_transferred_with_model -> diesel::sql_types::Integer,
    pipette_tip_model -> diesel::sql_types::Integer, procedure_template_pipette_tip_model
    -> diesel::sql_types::Integer }
}
