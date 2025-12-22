diesel::table! {
    freeze_drying_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, pascal ->
    diesel::sql_types::Float, seconds -> diesel::sql_types::Float,
    freeze_dried_with_model -> diesel::sql_types::Integer,
    procedure_template_freeze_dried_with_model -> diesel::sql_types::Integer,
    freeze_dried_container_model -> diesel::sql_types::Integer,
    procedure_template_freeze_dried_container_model -> diesel::sql_types::Integer }
}
