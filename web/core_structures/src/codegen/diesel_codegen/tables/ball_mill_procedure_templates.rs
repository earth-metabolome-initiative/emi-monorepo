diesel::table! {
    ball_mill_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, kelvin -> diesel::sql_types::Float,
    kelvin_tolerance_percentage -> diesel::sql_types::Float, seconds ->
    diesel::sql_types::Float, hertz -> diesel::sql_types::Float, bead_model ->
    diesel::sql_types::Integer, procedure_template_bead_model ->
    diesel::sql_types::Integer, number_of_beads -> diesel::sql_types::SmallInt,
    milled_with_model -> diesel::sql_types::Integer, procedure_template_milled_with_model
    -> diesel::sql_types::Integer, milled_container_model -> diesel::sql_types::Integer,
    foreign_procedure_template -> diesel::sql_types::Integer,
    procedure_template_milled_container_model -> diesel::sql_types::Integer }
}
