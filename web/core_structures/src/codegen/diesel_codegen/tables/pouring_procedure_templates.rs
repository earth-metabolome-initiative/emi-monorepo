diesel::table! {
    pouring_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, measured_with_model -> diesel::sql_types::Integer,
    procedure_template_measured_with_model -> diesel::sql_types::Integer,
    poured_from_model -> diesel::sql_types::Integer, foreign_procedure_template ->
    diesel::sql_types::Integer, procedure_template_poured_from_model ->
    diesel::sql_types::Integer, poured_into_model -> diesel::sql_types::Integer,
    procedure_template_poured_into_model -> diesel::sql_types::Integer, liters ->
    diesel::sql_types::Float }
}
