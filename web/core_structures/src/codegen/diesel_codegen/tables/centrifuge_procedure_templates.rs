diesel::table! {
    centrifuge_procedure_templates(procedure_template_id) { procedure_template ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, rotation_per_minute -> diesel::sql_types::Float,
    centrifuged_with_model -> diesel::sql_types::Integer,
    procedure_template_centrifuged_with_model -> diesel::sql_types::Integer,
    centrifuged_container_model -> diesel::sql_types::Integer,
    procedure_template_centrifuged_container_model -> diesel::sql_types::Integer }
}
