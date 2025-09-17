diesel::table! {
    fractioning_procedure_templates(procedure_template) { procedure_template ->
    diesel::sql_types::Integer, kilograms -> diesel::sql_types::Float,
    tolerance_percentage -> diesel::sql_types::Float, weighed_with_model ->
    diesel::sql_types::Integer, procedure_template_weighed_with_model ->
    diesel::sql_types::Integer, fragment_container_model -> diesel::sql_types::Integer,
    procedure_template_fragment_container_model -> diesel::sql_types::Integer,
    fragment_placed_into_model -> diesel::sql_types::Integer,
    procedure_template_fragment_placed_into_model -> diesel::sql_types::Integer }
}
